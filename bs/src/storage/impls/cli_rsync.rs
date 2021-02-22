use anyhow::anyhow;
use std::process::Command;

use crate::config::ERR_MSG_CONF_INVARIANT_NOT_MAINTAINED;
use crate::storage::{config::StorageConfig, StorageSyncer, SyncResult};

pub struct CLIRSyncStorageSyncer {
    config: StorageConfig,
}

impl CLIRSyncStorageSyncer {
    pub fn new(config: StorageConfig) -> CLIRSyncStorageSyncer {
        CLIRSyncStorageSyncer { config }
    }
}

impl StorageSyncer for CLIRSyncStorageSyncer {
    fn sync(&self) -> SyncResult {
        for i in 0..self.config.client_paths.len() {
            let sync_from = self
                .config
                .client_paths
                .get(i)
                .ok_or(anyhow!(ERR_MSG_CONF_INVARIANT_NOT_MAINTAINED))?;

            let sync_to = format!(
                "{}@{}:{:?}",
                self.config.server_user,
                self.config.server_ip,
                self.config
                    .server_paths
                    .get(i)
                    .ok_or(anyhow!(ERR_MSG_CONF_INVARIANT_NOT_MAINTAINED))?,
            );

            let output = Command::new("rsync")
                .arg("--verbose")
                .arg("--archive")
                .arg("--recursive")
                .arg(sync_from)
                .arg(sync_to)
                .output()?;

            println!("output: {:?}", output);
        }

        Ok(())
    }
}
