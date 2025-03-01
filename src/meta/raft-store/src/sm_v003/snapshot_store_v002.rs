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

use std::fmt::Display;
use std::io;
use std::str::FromStr;

use databend_common_meta_stoerr::MetaStorageError;
use databend_common_meta_types::raft_types::ErrorSubject;
use databend_common_meta_types::raft_types::StorageError;
use databend_common_meta_types::SnapshotData;
use log::error;
use log::info;
use log::warn;
use openraft::AnyError;
use openraft::ErrorVerb;
use openraft::SnapshotId;

use crate::config::RaftConfig;
use crate::ondisk::DataVersion;
use crate::snapshot_config::SnapshotConfig;
use crate::state_machine::MetaSnapshotId;

/// Errors that occur when accessing snapshot store
#[derive(Debug, thiserror::Error)]
#[error("SnapshotStoreError({verb:?}: {source}, while: {context}")]
pub struct SnapshotStoreError {
    verb: ErrorVerb,

    #[source]
    source: io::Error,
    context: String,
}

impl SnapshotStoreError {
    pub fn read(error: io::Error) -> Self {
        Self {
            verb: ErrorVerb::Read,
            source: error,
            context: "".to_string(),
        }
    }

    pub fn write(error: io::Error) -> Self {
        Self {
            verb: ErrorVerb::Write,
            source: error,
            context: "".to_string(),
        }
    }

    pub fn add_context(&mut self, context: impl Display) {
        if self.context.is_empty() {
            self.context = context.to_string();
        } else {
            self.context = format!("{}; while {}", self.context, context);
        }
    }

    pub fn with_context(mut self, context: impl Display) -> Self {
        self.add_context(context);
        self
    }

    /// Add meta and context info to the error.
    ///
    /// meta is anything that can be displayed.
    pub fn with_meta(self, context: impl Display, meta: impl Display) -> Self {
        self.with_context(format_args!("{}: {}", context, meta))
    }
}

impl From<SnapshotStoreError> for StorageError {
    fn from(error: SnapshotStoreError) -> Self {
        StorageError::new(
            ErrorSubject::Snapshot(None),
            error.verb,
            AnyError::new(&error),
        )
    }
}

impl From<SnapshotStoreError> for MetaStorageError {
    fn from(value: SnapshotStoreError) -> Self {
        MetaStorageError::damaged(&value.source, || {
            format!("when {}: {}", value.verb, value.context)
        })
    }
}

#[derive(Debug)]
pub struct SnapshotStoreV002 {
    snapshot_config: SnapshotConfig,
}

impl SnapshotStoreV002 {
    pub fn new(config: RaftConfig) -> Self {
        SnapshotStoreV002 {
            snapshot_config: SnapshotConfig::new(DataVersion::V002, config),
        }
    }

    pub fn data_version(&self) -> DataVersion {
        self.snapshot_config.data_version()
    }

    pub fn snapshot_config(&self) -> &SnapshotConfig {
        &self.snapshot_config
    }

    pub fn config(&self) -> &RaftConfig {
        self.snapshot_config.raft_config()
    }

    /// Return a list of valid snapshot ids found in the snapshot directory.
    pub async fn load_last_snapshot(
        &self,
    ) -> Result<Option<(MetaSnapshotId, SnapshotData)>, SnapshotStoreError> {
        let (snapshot_ids, _invalid_files) = self.load_snapshot_ids().await?;
        // dbg!(&snapshot_ids);

        info!("choose the latest from found snapshots: {:?}", snapshot_ids);

        let id = if let Some(id) = snapshot_ids.last().cloned() {
            id
        } else {
            return Ok(None);
        };

        let data = self.load_snapshot(&id.to_string()).await?;

        Ok(Some((id, data)))
    }

    /// Return a list of valid snapshot ids and invalid file names found in the snapshot directory.
    ///
    /// The valid snapshot ids are sorted, older first.
    pub async fn load_snapshot_ids(
        &self,
    ) -> Result<(Vec<MetaSnapshotId>, Vec<String>), SnapshotStoreError> {
        let mut snapshot_ids = vec![];
        let mut invalid_files = vec![];

        let dir = self
            .snapshot_config
            .ensure_snapshot_dir()
            .map_err(SnapshotStoreError::write)?;

        let mut read_dir = tokio::fs::read_dir(&dir)
            .await
            .map_err(|e| Self::make_err(e, format_args!("reading snapshot dir: {}", &dir)))?;

        while let Some(dent) = read_dir
            .next_entry()
            .await
            .map_err(|e| Self::make_err(e, format_args!("reading snapshot dir entry: {}", &dir)))?
        {
            let file_name = if let Some(x) = dent.file_name().to_str() {
                x.to_string()
            } else {
                continue;
            };

            if let Some(snapshot_id_str) = Self::extract_snapshot_id_from_fn(&file_name) {
                let meta_snap_id = if let Ok(x) = MetaSnapshotId::from_str(snapshot_id_str) {
                    x
                } else {
                    warn!("found invalid snapshot id: {}", file_name);
                    invalid_files.push(file_name);
                    continue;
                };

                snapshot_ids.push(meta_snap_id);
            }
        }

        snapshot_ids.sort();
        invalid_files.sort();

        info!("dir: {}; loaded snapshots: {:?}", dir, snapshot_ids);
        info!("dir: {}; invalid files: {:?}", dir, invalid_files);

        Ok((snapshot_ids, invalid_files))
    }

    /// Return a snapshot for async reading
    pub async fn load_snapshot(
        &self,
        snapshot_id: &SnapshotId,
    ) -> Result<SnapshotData, SnapshotStoreError> {
        self.snapshot_config
            .ensure_snapshot_dir()
            .map_err(SnapshotStoreError::write)?;

        let path = self.snapshot_config.snapshot_path(snapshot_id);

        let d = SnapshotData::open(path.clone()).map_err(|e| {
            error!("failed to open snapshot file({}): {}", path, e);
            SnapshotStoreError::read(e).with_meta("opening snapshot file", path)
        })?;

        Ok(d)
    }

    fn extract_snapshot_id_from_fn(filename: &str) -> Option<&str> {
        if let Some(snapshot_id) = filename.strip_suffix(".snap") {
            Some(snapshot_id)
        } else {
            None
        }
    }

    /// Build a [`SnapshotStoreError`] from io::Error with context.
    fn make_err(e: io::Error, context: impl Display) -> SnapshotStoreError {
        let s = context.to_string();
        error!("{} while context: {}", e, s);
        SnapshotStoreError::read(e).with_context(context)
    }
}
