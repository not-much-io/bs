use std::{net::IpAddr, path::PathBuf};

use crate::config::BsConfig;

pub struct StorageConfig {
    pub server_ip:    IpAddr,
    pub server_user:  String,
    pub server_paths: Vec<PathBuf>,
    pub client_paths: Vec<PathBuf>,
}

impl From<BsConfig> for StorageConfig {
    fn from(config: BsConfig) -> Self {
        StorageConfig {
            server_ip:    config.server_ip,
            server_user:  config.server_user,
            server_paths: config.server_paths,
            client_paths: config.client_paths,
        }
    }
}
