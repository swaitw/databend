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
use std::io::Write;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use databend_common_column::buffer::Buffer;
use databend_common_expression::types::Bitmap;

use super::compress_sample_ratio;
use super::IntegerCompression;
use super::IntegerStats;
use super::IntegerType;
use crate::compression::is_valid;
use crate::compression::Compression;
use crate::compression::SAMPLE_COUNT;
use crate::compression::SAMPLE_SIZE;
use crate::error::Result;
use crate::write::WriteOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rle {}

impl<T: IntegerType> IntegerCompression<T> for Rle {
    fn compress(
        &self,
        array: &Buffer<T>,
        stats: &IntegerStats<T>,
        _write_options: &WriteOptions,
        output: &mut Vec<u8>,
    ) -> Result<usize> {
        let size = output.len();
        self.compress_integer(output, array.clone(), stats.validity.clone())?;
        Ok(output.len() - size)
    }

    fn decompress(&self, input: &[u8], length: usize, output: &mut Vec<T>) -> Result<()> {
        let _ = self.decompress_integer(input, length, output)?;
        Ok(())
    }

    fn to_compression(&self) -> Compression {
        Compression::Rle
    }

    fn compress_ratio(&self, stats: &IntegerStats<T>) -> f64 {
        compress_sample_ratio(self, stats, SAMPLE_COUNT, SAMPLE_SIZE)
    }
}

impl Rle {
    pub fn compress_integer<T: IntegerType, W: Write>(
        &self,
        w: &mut W,
        values: impl IntoIterator<Item = T>,
        validity: Option<Bitmap>,
    ) -> Result<()> {
        // help me generate RLE encode algorithm
        let mut seen_count: u32 = 0;
        let mut last_value = T::default();
        let mut all_null = true;

        for (i, item) in values.into_iter().enumerate() {
            if is_valid(validity.as_ref(), i) {
                if all_null {
                    all_null = false;
                    last_value = item;

                    seen_count += 1;
                } else if last_value != item {
                    // flush  u32 cnt , value
                    w.write_all(&seen_count.to_le_bytes())?;
                    w.write_all(last_value.to_le_bytes().as_ref())?;

                    last_value = item;
                    seen_count = 1;
                } else {
                    seen_count += 1;
                }
            } else {
                // NULL value: we merely increment the seen_count
                seen_count += 1;
            }
        }

        if seen_count != 0 {
            w.write_all(&seen_count.to_le_bytes())?;
            w.write_all(last_value.to_le_bytes().as_ref())?;
        }

        Ok(())
    }

    pub fn decompress_integer<'a, T: IntegerType>(
        &self,
        mut input: &'a [u8],
        length: usize,
        array: &mut Vec<T>,
    ) -> Result<&'a [u8]> {
        array.reserve(length);
        let mut bs = vec![0u8; std::mem::size_of::<T>()];
        let mut num_values = 0;
        loop {
            let len = input.read_u32::<LittleEndian>()?;
            input.read_exact(&mut bs)?;

            let a: T::Bytes = match bs.as_slice().try_into() {
                Ok(a) => a,
                Err(_) => unreachable!(),
            };
            let t = T::from_le_bytes(a);
            for _ in 0..len {
                array.push(t);
            }

            num_values += len as usize;
            if num_values >= length {
                break;
            }
        }
        Ok(input)
    }
}
