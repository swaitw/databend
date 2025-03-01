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

use databend_common_exception::ErrorCode;
use databend_common_exception::Result;
use databend_common_expression::Scalar;
use databend_common_expression::TableField;

use crate::hive_partition::parse_hive_partitions;
use crate::utils::str_field_to_scalar;

#[derive(Debug, Clone)]
pub struct HivePartitionFiller {
    pub partition_fields: Vec<TableField>,
}

impl HivePartitionFiller {
    pub fn create(partition_fields: Vec<TableField>) -> Self {
        HivePartitionFiller { partition_fields }
    }

    pub fn extract_scalars(&self, locations: &str) -> Result<Vec<Scalar>> {
        let partition_map = parse_hive_partitions(locations);

        let mut partition_values = vec![];
        for field in self.partition_fields.iter() {
            match partition_map.get(field.name()) {
                Some(v) => {
                    let value = str_field_to_scalar(v.as_str(), &field.data_type().into())?;
                    partition_values.push(value);
                }
                None => {
                    return Err(ErrorCode::TableInfoError(format!(
                        "couldn't find hive partition info :{}, hive partition maps:{:?}",
                        field.name(),
                        partition_map
                    )));
                }
            };
        }
        Ok(partition_values)
    }
}
