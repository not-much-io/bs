use std::net::IpAddr;

pub struct Ssh {}

impl Ssh {
    pub fn test(&self) {}
    pub fn run_on(&self) {}
}

pub struct SshConfig {
    ip:   IpAddr,
    user: String,
}

impl SshConfig {
    pub fn new(ip: IpAddr, user: String) -> Self {
        SshConfig { ip, user }
    }
}
