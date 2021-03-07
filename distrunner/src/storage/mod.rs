use std::{collections::HashMap, net::IpAddr, path::PathBuf, process::Command};

use anyhow::{anyhow, Error, Result};

pub type SyncResult = Result<(), Error>;

pub struct StorageSyncer {
    pub server_ip:     IpAddr,
    pub server_user:   String,
    pub path_mappings: HashMap<PathBuf, PathBuf>,
}

impl StorageSyncer {
    pub fn new(
        server_ip: IpAddr,
        server_user: String,
        path_mappings: HashMap<PathBuf, PathBuf>,
    ) -> Self {
        StorageSyncer {
            server_ip,
            server_user,
            path_mappings,
        }
    }

    pub fn sync(&self) -> SyncResult {
        for (client_path, server_path) in self.path_mappings.iter() {
            let sync_from = client_path;
            let sync_to = format!("{}@{}:{:?}", self.server_user, self.server_ip, server_path,);

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
