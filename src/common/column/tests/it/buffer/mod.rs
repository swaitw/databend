// Copyright 2020-2022 Jorge C. Leitão
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

use databend_common_column::binview::BinaryViewColumnBuilder;
use databend_common_column::buffer::Buffer;

mod immutable;

#[test]
fn new_basic() {
    let mut buffer = Buffer::<i32>::new();
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());

    buffer = Buffer::<i32>::from(vec![1, 2, 3]);
    assert_eq!(buffer.len(), 3);
}

#[test]
fn extend_from_repeats() {
    let mut b = BinaryViewColumnBuilder::<str>::new();
    b.extend_constant(4, "databend");

    let a = b.clone();
    b.extend_trusted_len_values(a.iter());

    assert_eq!(
        b.freeze(),
        BinaryViewColumnBuilder::<str>::from_values_iter(vec!["databend"; 8].into_iter()).freeze()
    )
}
