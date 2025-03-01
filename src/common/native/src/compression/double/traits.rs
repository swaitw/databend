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

use std::hash::Hash;
use std::ops::BitXor;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;

use databend_common_column::types::NativeType;
use databend_common_expression::types::F32;
use databend_common_expression::types::F64;
use num::Float;

use crate::util::AsBytes;

pub trait DoubleType:
    AsBytes + Copy + Clone + NativeType + Float + Ord + Hash + PartialOrd
{
    type RawType: std::fmt::Debug + std::fmt::Display + Copy + Clone;

    type BitType: Eq
        + NativeType
        + Hash
        + PartialOrd
        + Hash
        + AsBytes
        + BitXor<Output = Self::BitType>
        + ShlAssign
        + Shl<usize, Output = Self::BitType>
        + Shr<usize, Output = Self::BitType>
        + ShrAssign;

    fn as_bits(&self) -> Self::BitType;
    fn from_bits_val(bits: Self::BitType) -> Self;

    fn leading_zeros(bit_value: &Self::BitType) -> u32;
    fn trailing_zeros(bit_value: &Self::BitType) -> u32;
}

macro_rules! double_type {
    ($type:ty, $raw_type: ty,  $bit_type: ty) => {
        impl DoubleType for $type {
            type RawType = $raw_type;
            type BitType = $bit_type;

            fn as_bits(&self) -> Self::BitType {
                self.0.to_bits()
            }

            fn from_bits_val(bits: Self::BitType) -> Self {
                Self::from(Self::RawType::from_bits(bits))
            }

            fn leading_zeros(bit_value: &Self::BitType) -> u32 {
                bit_value.leading_zeros()
            }

            fn trailing_zeros(bit_value: &Self::BitType) -> u32 {
                bit_value.trailing_zeros()
            }
        }
    };
}

double_type!(F32, f32, u32);
double_type!(F64, f64, u64);
