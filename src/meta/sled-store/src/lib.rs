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

#![allow(clippy::uninlined_format_args)]

//! sled_store implement a key-value like store backed by sled::Tree.
//!
//! It is used by raft for log and state machine storage.
pub use bytes_error::SledBytesError;
pub use db::drop_sled_db;
pub use db::get_sled_db;
pub use db::init_get_sled_db;
pub use db::init_sled_db;
pub use openraft;
pub use sled;
pub use sled_key_space::SledKeySpace;
pub use sled_serde::SledOrderedSerde;
pub use sled_serde::SledSerde;
pub use sled_tree::AsKeySpace;
pub use sled_tree::SledItem;
pub use sled_tree::SledTree;

mod bytes_error;
mod db;
mod sled_key_space;
mod sled_serde;
mod sled_serde_impl;
mod sled_tree;
