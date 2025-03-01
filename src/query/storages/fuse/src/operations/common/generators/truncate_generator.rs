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

use std::any::Any;
use std::sync::Arc;

use databend_common_exception::Result;
use databend_common_expression::TableSchema;
use databend_storages_common_table_meta::meta::TableSnapshot;
use uuid::Uuid;

use crate::operations::common::SnapshotGenerator;

#[derive(Clone)]
pub enum TruncateMode {
    // Truncate and keep the historical data.
    Normal,
    // Delete the data, used for delete operation.
    Delete,
    // Truncate and purge the historical data.
    Purge,
}

#[derive(Clone)]
pub struct TruncateGenerator {
    mode: TruncateMode,
}

impl TruncateGenerator {
    pub fn new(mode: TruncateMode) -> Self {
        TruncateGenerator { mode }
    }

    pub fn mode(&self) -> &TruncateMode {
        &self.mode
    }
}

#[async_trait::async_trait]
impl SnapshotGenerator for TruncateGenerator {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn do_generate_new_snapshot(
        &self,
        schema: TableSchema,
        _cluster_key_id: Option<u32>,
        previous: &Option<Arc<TableSnapshot>>,
        prev_table_seq: Option<u64>,
        _table_name: &str,
    ) -> Result<TableSnapshot> {
        let (prev_timestamp, prev_snapshot_id) = if let Some(prev_snapshot) = previous {
            (
                prev_snapshot.timestamp,
                Some((prev_snapshot.snapshot_id, prev_snapshot.format_version)),
            )
        } else {
            (None, None)
        };

        let new_snapshot = TableSnapshot::new(
            Uuid::new_v4(),
            prev_table_seq,
            &prev_timestamp,
            prev_snapshot_id,
            schema,
            Default::default(),
            vec![],
            None,
        );
        Ok(new_snapshot)
    }
}
