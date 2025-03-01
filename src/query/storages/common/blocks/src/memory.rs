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

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;

use databend_common_expression::DataBlock;
use parking_lot::RwLock;
/// Shared store to support memory tables.
///
/// Indexed by table id etc.
pub type InMemoryData<K> = HashMap<K, Arc<RwLock<Vec<DataBlock>>>>;

pub static IN_MEMORY_DATA: LazyLock<Arc<RwLock<InMemoryData<InMemoryDataKey>>>> =
    LazyLock::new(|| Arc::new(Default::default()));

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct InMemoryDataKey {
    pub temp_prefix: Option<String>,
    pub table_id: u64,
}
