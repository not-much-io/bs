use std::{collections::HashMap, net::IpAddr, path::PathBuf};

pub struct BsConfig {
    pub client_ip:   IpAddr,
    pub client_user: String,

    pub server_ip:   IpAddr,
    pub server_user: String,

    pub path_mappings: HashMap<PathBuf, PathBuf>,
}

impl BsConfig {
    pub fn new(
        client_ip: IpAddr,
        client_user: String,
        server_ip: IpAddr,
        server_user: String,
        path_mappings: HashMap<PathBuf, PathBuf>,
    ) -> BsConfig {
        BsConfig {
            client_ip,
            client_user,
            server_ip,
            server_user,
            path_mappings,
        }
    }
}
