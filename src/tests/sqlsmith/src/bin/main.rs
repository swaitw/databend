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

use clap::Parser;
use databend_common_exception::Result;
use databend_sqlsmith::Runner;
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
#[clap(about, author)]
pub struct Args {
    /// The database host.
    #[clap(long, default_value = "localhost")]
    host: String,

    /// The database http port.
    #[clap(long, default_value = "8000")]
    port: u16,

    /// The test database.
    #[clap(long, default_value = "sqlsmith_test")]
    db: String,

    /// The username.
    #[clap(long, default_value = "root")]
    user: String,

    /// The password.
    #[clap(long, default_value = "")]
    pass: String,

    /// The number of test cases to generate.
    #[clap(long, default_value = "500")]
    count: usize,

    /// The number of timeout seconds of one query.
    #[clap(long, default_value = "5")]
    timeout: u64,

    /// The fuzz query test file path.
    #[clap(long, default_value = "")]
    fuzz_path: String,

    /// The log path to write executed SQLs..
    #[clap(long, default_value = ".databend/sqlsmithlog")]
    log_path: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() -> Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse("")
        .unwrap();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let args = Args::parse();
    let host = format!("http://{}:{}", args.host, args.port);
    let mut runner = Runner::try_new(
        host,
        args.user.clone(),
        args.pass.clone(),
        args.db.clone(),
        args.log_path.clone(),
        args.count,
        None,
        args.timeout,
    )
    .await?;

    if !args.fuzz_path.is_empty() {
        runner.run_fuzz(&args.fuzz_path).await?;
    } else {
        runner.run().await?;
    }

    Ok(())
}
