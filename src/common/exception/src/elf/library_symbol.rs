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
use std::fmt::Debug;
use std::fmt::Formatter;

pub struct Symbol {
    pub name: &'static [u8],
    pub address_end: u64,
    pub address_begin: u64,
}

impl Symbol {
    pub fn sort_begin_address(&self, other: &Self) -> Ordering {
        self.address_begin.cmp(&other.address_begin)
    }

    pub fn same_address(&mut self, other: &mut Self) -> bool {
        self.address_begin == other.address_begin && self.address_end == other.address_end
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Symbol")
            .field(
                "name",
                &rustc_demangle::demangle(std::str::from_utf8(self.name).unwrap()),
            )
            .field("address_begin", &self.address_begin)
            .field("address_end", &self.address_end)
            .finish()
    }
}
