use std::net::IpAddr;

pub struct ProcessingConfig {
    pub server_ip:   IpAddr,
    pub server_user: String,
    pub commands:    Vec<String>,
}
