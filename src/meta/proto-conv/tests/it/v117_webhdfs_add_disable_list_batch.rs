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

use chrono::DateTime;
use chrono::Utc;
use databend_common_meta_app as mt;
use databend_common_meta_app::principal::UserIdentity;
use databend_common_meta_app::storage::StorageParams;
use databend_common_meta_app::storage::StorageWebhdfsConfig;
use fastrace::func_name;

use crate::common;

// These bytes are built when a new version in introduced,
// and are kept for backward compatibility test.
//
// *************************************************************
// * These messages should never be updated,                   *
// * only be added when a new version is added,                *
// * or be removed when an old version is no longer supported. *
// *************************************************************
//
#[test]
fn test_v117_webhdfs_add_disable_list_batch() -> anyhow::Result<()> {
    // Encoded data of version 117 of databend_common_meta_app::principal::user_stage::StageInfo:
    // It is generated with common::test_pb_from_to().
    let stage_info_v117 = vec![
        10, 22, 119, 101, 98, 104, 100, 102, 115, 58, 47, 47, 100, 105, 114, 47, 116, 111, 47, 102,
        105, 108, 101, 115, 16, 1, 26, 12, 10, 10, 42, 8, 48, 1, 160, 6, 117, 168, 6, 24, 42, 11,
        10, 2, 48, 2, 16, 142, 8, 24, 1, 56, 1, 50, 4, 116, 101, 115, 116, 56, 100, 66, 29, 10, 8,
        100, 97, 116, 97, 98, 101, 110, 100, 18, 11, 100, 97, 116, 97, 98, 101, 110, 100, 46, 114,
        115, 160, 6, 117, 168, 6, 24, 74, 10, 34, 8, 8, 2, 160, 6, 117, 168, 6, 24, 82, 23, 49, 57,
        55, 48, 45, 48, 49, 45, 48, 49, 32, 48, 48, 58, 48, 48, 58, 48, 48, 32, 85, 84, 67, 160, 6,
        117, 168, 6, 24,
    ];

    let want = || mt::principal::StageInfo {
        stage_name: "webhdfs://dir/to/files".to_string(),
        stage_type: mt::principal::StageType::External,
        stage_params: mt::principal::StageParams {
            storage: StorageParams::Webhdfs(StorageWebhdfsConfig {
                disable_list_batch: true,
                ..Default::default()
            }),
        },
        is_temporary: false,
        file_format_params: mt::principal::FileFormatParams::Json(
            mt::principal::JsonFileFormatParams {
                compression: mt::principal::StageFileCompression::Bz2,
            },
        ),
        copy_options: mt::principal::CopyOptions {
            on_error: mt::principal::OnErrorMode::AbortNum(2),
            size_limit: 1038,
            max_files: 0,
            split_size: 0,
            purge: true,
            single: false,
            max_file_size: 0,
            disable_variant_check: true,
            return_failed_only: false,
            detailed_output: false,
        },
        comment: "test".to_string(),
        number_of_files: 100,
        creator: Some(UserIdentity {
            username: "databend".to_string(),
            hostname: "databend.rs".to_string(),
        }),
        created_on: DateTime::<Utc>::default(),
    };

    common::test_pb_from_to(func_name!(), want())?;
    common::test_load_old(func_name!(), stage_info_v117.as_slice(), 117, want())?;

    Ok(())
}
