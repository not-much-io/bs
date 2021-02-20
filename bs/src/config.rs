use std::{net::IpAddr, path::PathBuf};

pub const ERR_MSG_CONF_INVARIANT_NOT_MAINTAINED: &str =
    "Some invariant of bs configuration unmaintained, this is a bug";

pub struct BsConfig {
    pub client_ip:    IpAddr,
    pub client_user:  String,
    pub client_paths: Vec<PathBuf>,

    pub server_ip:    IpAddr,
    pub server_user:  String,
    pub server_paths: Vec<PathBuf>,
}

impl BsConfig {
    pub fn new(
        client_ip: IpAddr,
        client_user: String,
        client_paths: Vec<PathBuf>,
        server_ip: IpAddr,
        server_user: String,
        server_paths: Vec<PathBuf>,
    ) -> BsConfig {
        // TODO: Valide invariants, return Result<thiserror::Error>

        BsConfig {
            client_ip,
            client_user,
            client_paths,
            server_ip,
            server_user,
            server_paths,
        }
    }
}
