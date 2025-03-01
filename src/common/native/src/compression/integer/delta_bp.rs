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

use std::cmp::Ordering;
use std::io::BufRead;

use bitpacking::BitPacker;
use bitpacking::BitPacker4x;
use byteorder::ReadBytesExt;
use databend_common_column::buffer::Buffer;

use super::compress_sample_ratio;
use super::IntegerCompression;
use super::IntegerStats;
use super::IntegerType;
use crate::compression::Compression;
use crate::compression::SAMPLE_COUNT;
use crate::compression::SAMPLE_SIZE;
use crate::error::Result;
use crate::write::WriteOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeltaBitpacking {}

impl<T: IntegerType> IntegerCompression<T> for DeltaBitpacking {
    fn compress(
        &self,
        array: &Buffer<T>,
        _stats: &IntegerStats<T>,
        _write_options: &WriteOptions,
        output: &mut Vec<u8>,
    ) -> Result<usize> {
        let start: usize = output.len();
        let bitpacker = BitPacker4x::new();
        let my_data = bytemuck::cast_slice(array.as_slice());

        let mut initial = 0;
        for chunk in my_data.chunks(BitPacker4x::BLOCK_LEN) {
            let num_bits: u8 = bitpacker.num_bits(chunk);
            output.push(num_bits);
            output.reserve(BitPacker4x::BLOCK_LEN * 4);

            let out_slice = unsafe {
                core::slice::from_raw_parts_mut(
                    output.as_mut_ptr().add(output.len()),
                    BitPacker4x::BLOCK_LEN * 4,
                )
            };

            let size = bitpacker.compress_sorted(initial, chunk, out_slice, num_bits);
            initial = *chunk.last().unwrap();
            unsafe { output.set_len(output.len() + size) };
        }

        Ok(output.len() - start)
    }

    fn decompress(&self, mut input: &[u8], length: usize, output: &mut Vec<T>) -> Result<()> {
        log::debug!("DeltaBitpacking::decompress {}", input.len());
        let bitpacker = BitPacker4x::new();

        let mut initial = 0;

        output.reserve(BitPacker4x::BLOCK_LEN * length);
        for _ in (0..length).step_by(BitPacker4x::BLOCK_LEN) {
            let num_bits = input.read_u8()?;
            let out_slice = unsafe {
                core::slice::from_raw_parts_mut(
                    output.as_mut_ptr().add(output.len()) as *mut u32,
                    BitPacker4x::BLOCK_LEN,
                )
            };
            let size = bitpacker.decompress_sorted(initial, input, out_slice, num_bits);
            input.consume(size);

            initial = *out_slice.last().unwrap();
            unsafe { output.set_len(output.len() + BitPacker4x::BLOCK_LEN) };
        }
        Ok(())
    }

    fn to_compression(&self) -> Compression {
        Compression::DeltaBitpacking
    }

    fn compress_ratio(&self, stats: &IntegerStats<T>) -> f64 {
        if match stats.min.compare_i64(0) {
            Ordering::Greater | Ordering::Equal => false,
            Ordering::Less => true,
        } || std::mem::size_of::<T>() != 4
            || stats.src.len() % BitPacker4x::BLOCK_LEN != 0
            || !stats.is_sorted
            || stats.null_count > 0
        {
            return 0.0f64;
        }

        let bpk = super::bp::Bitpacking {};
        compress_sample_ratio(&bpk, stats, SAMPLE_COUNT, SAMPLE_SIZE) * 1.50f64
    }
}
