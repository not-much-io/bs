use std::{collections::HashMap, net::IpAddr, path::PathBuf};

use crate::config::BsConfig;

pub struct StorageConfig {
    pub server_ip:     IpAddr,
    pub server_user:   String,
    pub path_mappings: HashMap<PathBuf, PathBuf>,
}

impl From<BsConfig> for StorageConfig {
    fn from(config: BsConfig) -> Self {
        StorageConfig {
            server_ip:     config.server_ip,
            server_user:   config.server_user,
            path_mappings: config.path_mappings,
        }
    }
}
