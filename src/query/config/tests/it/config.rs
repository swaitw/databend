// Copyright 2023 Datafuse Labs.
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

use std::ffi::OsString;

use clap::Parser;
use databend_common_config::Config;
use databend_common_config::InnerConfig;
use pretty_assertions::assert_eq;

/// It's required to make sure setting's default value is the same with clap.
#[test]
fn test_config_default() {
    let setting_default = InnerConfig::default();
    let config_default: InnerConfig = Config::parse_from(Vec::<OsString>::new())
        .try_into()
        .expect("parse from args must succeed");

    assert_eq!(
        setting_default, config_default,
        "default setting is different from default config, please check again"
    )
}
