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

#![feature(duration_constants)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::diverging_sub_expression)]

mod converters;
mod hive_catalog;
mod hive_database;
mod hive_partition;
mod hive_partition_filler;
mod hive_table;
mod hive_table_options;
mod hive_table_source;
mod utils;

pub use hive_catalog::HiveCatalog;
pub use hive_catalog::HiveCreator;
pub use hive_partition::HivePartInfo;
pub use hive_partition_filler::HivePartitionFiller;
pub use hive_table::HiveTable;
