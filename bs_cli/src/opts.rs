use bs::config::BsConfig;
use clap::Clap;
use std::{net::IpAddr, path::PathBuf};

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Kristo Koert <kristo.koert@gmail.com>")]
pub struct Opts {
    #[clap(long)]
    pub client_ip:    IpAddr,
    #[clap(long)]
    pub client_user:  String,
    #[clap(long, required(true), multiple(true))]
    pub client_paths: Vec<PathBuf>,

    #[clap(long)]
    pub server_ip:    IpAddr,
    #[clap(long)]
    pub server_user:  String,
    #[clap(long, required(true), multiple(true))]
    pub server_paths: Vec<PathBuf>,
}

impl From<Opts> for BsConfig {
    fn from(opts: Opts) -> Self {
        BsConfig::new(
            opts.client_ip,
            opts.client_user,
            opts.client_paths,
            opts.server_ip,
            opts.server_user,
            opts.server_paths,
        )
    }
}
