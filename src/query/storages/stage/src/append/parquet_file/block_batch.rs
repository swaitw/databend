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

use databend_common_expression::local_block_meta_serde;
use databend_common_expression::BlockMetaInfo;
use databend_common_expression::DataBlock;

#[derive(Debug)]
pub struct BlockBatch {
    pub blocks: Vec<DataBlock>,
}

impl BlockBatch {
    pub fn create_block(blocks: Vec<DataBlock>) -> DataBlock {
        DataBlock::empty_with_meta(Box::new(BlockBatch { blocks }))
    }
}

impl Clone for BlockBatch {
    fn clone(&self) -> Self {
        unreachable!("Buffers should not be cloned")
    }
}

local_block_meta_serde!(BlockBatch);

#[typetag::serde(name = "unload_block_batch")]
impl BlockMetaInfo for BlockBatch {}
