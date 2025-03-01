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

mod mysql_federated;
mod mysql_handler;
mod mysql_interactive_worker;
mod mysql_session;
#[allow(clippy::unused_io_amount)]
mod reject_connection;
mod tls;
mod writers;

pub use self::mysql_federated::MySQLFederated;
pub use self::mysql_handler::MySQLHandler;
pub use self::mysql_session::MySQLConnection;
pub use self::tls::MySQLTlsConfig;

const MYSQL_VERSION: &str = "8.0.90";
