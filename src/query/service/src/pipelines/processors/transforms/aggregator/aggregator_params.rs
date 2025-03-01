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

use std::sync::Arc;

use databend_common_exception::Result;
use databend_common_expression::types::DataType;
use databend_common_expression::ColumnBuilder;
use databend_common_expression::DataBlock;
use databend_common_expression::DataSchemaRef;
use databend_common_functions::aggregates::get_states_layout;
use databend_common_functions::aggregates::AggregateFunctionRef;
use databend_common_functions::aggregates::StatesLayout;
use databend_common_sql::IndexType;
use itertools::Itertools;

pub struct AggregatorParams {
    pub input_schema: DataSchemaRef,
    pub group_columns: Vec<IndexType>,
    pub group_data_types: Vec<DataType>,

    pub aggregate_functions: Vec<AggregateFunctionRef>,
    pub aggregate_functions_arguments: Vec<Vec<usize>>,

    // about function state memory layout
    // If there is no aggregate function, layout is None
    pub states_layout: Option<StatesLayout>,

    pub enable_experimental_aggregate_hashtable: bool,
    pub cluster_aggregator: bool,
    pub max_block_size: usize,
    pub max_spill_io_requests: usize,
}

impl AggregatorParams {
    pub fn try_create(
        input_schema: DataSchemaRef,
        group_data_types: Vec<DataType>,
        group_columns: &[usize],
        agg_funcs: &[AggregateFunctionRef],
        agg_args: &[Vec<usize>],
        enable_experimental_aggregate_hashtable: bool,
        cluster_aggregator: bool,
        max_block_size: usize,
        max_spill_io_requests: usize,
    ) -> Result<Arc<AggregatorParams>> {
        let states_layout = if !agg_funcs.is_empty() {
            Some(get_states_layout(agg_funcs)?)
        } else {
            None
        };

        Ok(Arc::new(AggregatorParams {
            input_schema,
            group_columns: group_columns.to_vec(),
            group_data_types,
            aggregate_functions: agg_funcs.to_vec(),
            aggregate_functions_arguments: agg_args.to_vec(),
            states_layout,
            enable_experimental_aggregate_hashtable,
            cluster_aggregator,
            max_block_size,
            max_spill_io_requests,
        }))
    }

    pub fn has_distinct_combinator(&self) -> bool {
        self.aggregate_functions
            .iter()
            .any(|f| f.name().contains("DistinctCombinator"))
    }

    pub fn empty_result_block(&self) -> DataBlock {
        let columns = self
            .aggregate_functions
            .iter()
            .map(|f| ColumnBuilder::with_capacity(&f.return_type().unwrap(), 0).build())
            .chain(
                self.group_data_types
                    .iter()
                    .map(|t| ColumnBuilder::with_capacity(t, 0).build()),
            )
            .collect_vec();
        DataBlock::new_from_columns(columns)
    }

    pub fn num_states(&self) -> usize {
        self.aggregate_functions.len()
    }
}
