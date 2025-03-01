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

pub use databend_common_column::error::Error;
pub use databend_common_column::error::Result;

#[macro_export]
macro_rules! general_err {
    ($fmt:expr) => (Error::OutOfSpec($fmt.to_owned()));
    ($fmt:expr, $($args:expr),*) => (Error::OutOfSpec(format!($fmt, $($args),*)));
    ($e:expr, $fmt:expr) => (Error::OutOfSpec($fmt.to_owned(), $e));
    ($e:ident, $fmt:expr, $($args:tt),*) => (
        Error::OutOfSpec(&format!($fmt, $($args),*), $e));
}

#[macro_export]
macro_rules! nyi_err {
    ($fmt:expr) => (Error::NotYetImplemented($fmt.to_owned()));
    ($fmt:expr, $($args:expr),*) => (Error::NotYetImplemented(format!($fmt, $($args),*)));
}
