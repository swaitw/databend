// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use databend_common_expression::TableSchema;
use opendal::Reader;

use super::read_basic::read_u32;
use super::read_basic::read_u64;
use super::NativeReadBuf;
use super::PageIterator;
use crate::error::Error;
use crate::error::Result;
use crate::ColumnMeta;
use crate::PageMeta;

const DEFAULT_FOOTER_SIZE: u64 = 64 * 1024;

#[derive(Debug)]
pub struct NativeReader<R: NativeReadBuf> {
    page_reader: R,
    page_metas: Vec<PageMeta>,
    current_page: usize,
    scratch: Vec<u8>,
}

impl<R: NativeReadBuf> NativeReader<R> {
    /// Creates a new [`NativeReader`]
    pub fn new(page_reader: R, page_metas: Vec<PageMeta>, scratch: Vec<u8>) -> Self {
        Self {
            page_reader,
            page_metas,
            current_page: 0,
            scratch,
        }
    }

    /// Check whether there is more data to read,
    /// returns true, if current page is not the last one, false otherwise
    pub fn has_next(&self) -> bool {
        self.current_page < self.page_metas.len()
    }

    /// Returns current page number
    pub fn current_page(&self) -> usize {
        self.current_page
    }
}

impl<R: NativeReadBuf> PageIterator for NativeReader<R> {
    fn swap_buffer(&mut self, scratch: &mut Vec<u8>) {
        std::mem::swap(&mut self.scratch, scratch)
    }
}

impl<R: NativeReadBuf + std::io::Seek> Iterator for NativeReader<R> {
    type Item = Result<(u64, Vec<u8>)>;

    /// Reads the next nth page of data, skipping the intermediate pages
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut i = 0;
        let mut length = 0;
        while i < n {
            if self.current_page == self.page_metas.len() {
                break;
            }
            let page_meta = &self.page_metas[self.current_page];
            length += page_meta.length;
            i += 1;
            self.current_page += 1;
        }
        if i < n {
            return None;
        }
        if length > 0 {
            if let Some(err) = self
                .page_reader
                .seek(SeekFrom::Current(length as i64))
                .err()
            {
                return Some(Result::Err(err.into()));
            }
        }
        self.next()
    }

    /// Reads the next page of data
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page == self.page_metas.len() {
            return None;
        }
        let mut buffer = std::mem::take(&mut self.scratch);
        let page_meta = &self.page_metas[self.current_page];
        buffer.resize(page_meta.length as usize, 0);
        if let Some(err) = self.page_reader.read_exact(&mut buffer).err() {
            return Some(Result::Err(err.into()));
        }
        self.current_page += 1;
        Some(Ok((page_meta.num_values, buffer)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.page_metas.len() - self.current_page;
        (remaining, Some(remaining))
    }
}

impl<R: NativeReadBuf + std::io::Seek> NativeReader<R> {
    /// Skips the next page
    pub fn skip_page(&mut self) -> Result<()> {
        if self.current_page == self.page_metas.len() {
            return Ok(());
        }
        let page_meta = &self.page_metas[self.current_page];
        self.page_reader
            .seek(SeekFrom::Current(page_meta.length as i64))?;
        self.current_page += 1;
        Ok(())
    }
}

fn deserialize_meta(buf: Vec<u8>) -> Result<Vec<ColumnMeta>> {
    let mut buf_reader = std::io::Cursor::new(buf);
    let mut buf = vec![0u8; 8];
    let meta_len = read_u64(&mut buf_reader, buf.as_mut_slice())?;
    let mut metas = Vec::with_capacity(meta_len as usize);
    for _i in 0..meta_len {
        let offset = read_u64(&mut buf_reader, buf.as_mut_slice())?;
        let page_num = read_u64(&mut buf_reader, buf.as_mut_slice())?;
        let mut pages = Vec::with_capacity(page_num as usize);
        for _p in 0..page_num {
            let length = read_u64(&mut buf_reader, buf.as_mut_slice())?;
            let num_values = read_u64(&mut buf_reader, buf.as_mut_slice())?;

            pages.push(PageMeta { length, num_values });
        }
        metas.push(ColumnMeta { offset, pages })
    }
    Ok(metas)
}

pub fn read_meta<Reader: Read + Seek>(reader: &mut Reader) -> Result<Vec<ColumnMeta>> {
    // EOS(8 bytes) + meta_size(4 bytes) = 12 bytes
    reader.seek(SeekFrom::End(-12))?;
    let mut buf = vec![0u8; 4];
    let meta_size = read_u32(reader, buf.as_mut_slice())? as usize;
    reader.seek(SeekFrom::End(-16 - meta_size as i64))?;

    let mut meta_buf = vec![0u8; meta_size];
    reader.read_exact(&mut meta_buf)?;
    deserialize_meta(meta_buf)
}

pub fn infer_schema<Reader: Read + Seek>(reader: &mut Reader) -> Result<TableSchema> {
    // EOS(8 bytes) + meta_size(4 bytes) + schema_size(4bytes) = 16 bytes
    reader.seek(SeekFrom::End(-16))?;
    let mut buf = vec![0u8; 4];
    let schema_size = read_u32(reader, buf.as_mut_slice())? as usize;
    let column_meta_size = read_u32(reader, buf.as_mut_slice())? as usize;

    reader.seek(SeekFrom::Current(
        -(column_meta_size as i64) - (schema_size as i64) - 8,
    ))?;
    let mut schema_buf = vec![0u8; schema_size];
    reader.read_exact(&mut schema_buf)?;
    let schema = serde_json::from_slice(&schema_buf).expect("deserialize schema error");
    Ok(schema)
}

pub async fn read_meta_async(
    reader: Reader,
    total_len: usize,
) -> Result<(Vec<ColumnMeta>, TableSchema)> {
    // Pre-read footer data to reduce IO.
    let pre_read_len = total_len.min(DEFAULT_FOOTER_SIZE as usize);

    let buf = reader
        .read(total_len as u64 - pre_read_len as u64..total_len as u64)
        .await
        .map_err(|err| Error::External("file read failed".to_string(), Box::new(err)))?;
    if buf.len() < pre_read_len {
        return Err(Error::OutOfSpec("file is too short".to_string()));
    }

    // EOS(8 bytes) + meta_size(4 bytes) + schema_size(4bytes) = 16 bytes
    let footer_size = 16;
    let mut footer_reader = std::io::Cursor::new(buf.to_bytes());
    footer_reader.seek(SeekFrom::End(-footer_size))?;
    let mut buf = vec![0u8; 4];
    let schema_size = read_u32(&mut footer_reader, buf.as_mut_slice())? as i64;
    let meta_size = read_u32(&mut footer_reader, buf.as_mut_slice())? as i64;

    let total_size = schema_size + meta_size + footer_size;
    if total_size > pre_read_len as i64 {
        // The readded data is not long enough to hold the meta data.
        // Should read again.
        let buf = reader
            .read(total_len as u64 - total_size as u64..total_len as u64)
            .await
            .map_err(|err| Error::External("file read failed".to_string(), Box::new(err)))?;
        if buf.len() < total_size as usize {
            return Err(Error::OutOfSpec("file is too short".to_string()));
        }
        footer_reader = std::io::Cursor::new(buf.to_bytes());
    } else {
        footer_reader.seek(SeekFrom::End(-total_size))?;
    }

    let mut schema_buf = vec![0u8; schema_size as usize];
    footer_reader.read_exact(&mut schema_buf)?;
    let schema = serde_json::from_slice(&schema_buf).expect("deserialize schema error");

    footer_reader.seek(SeekFrom::End(-footer_size - meta_size))?;
    let mut meta_buf = vec![0u8; meta_size as usize];
    footer_reader.read_exact(&mut meta_buf)?;
    let meta = deserialize_meta(meta_buf)?;
    Ok((meta, schema))
}
