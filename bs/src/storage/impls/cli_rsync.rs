use std::process::Command;

use crate::storage::{config::StorageConfig, StorageSyncer, SyncResult};
use anyhow::anyhow;

pub struct CliRSyncStorageSyncer {
    config: StorageConfig,
}

impl CliRSyncStorageSyncer {
    pub fn new(config: StorageConfig) -> Self {
        CliRSyncStorageSyncer { config }
    }
}

impl StorageSyncer for CliRSyncStorageSyncer {
    fn sync(&self) -> SyncResult {
        for (client_path, server_path) in self.config.path_mappings.iter() {
            let sync_from = client_path;
            let sync_to = format!(
                "{}@{}:{:?}",
                self.config.server_user, self.config.server_ip, server_path,
            );

            let output = Command::new("rsync")
                .arg("--verbose")
                .arg("--archive")
                .arg("--recursive")
                .arg(sync_from)
                .arg(sync_to)
                .output()?;

            if !output.stderr.is_empty() {
                return Err(anyhow!(
                    "rsync executon resulted in an error: {:?}",
                    output.stderr
                ));
            }
        }

        Ok(())
    }
}
