use anyhow::Result;
use bs::config::BsConfig;
use clap::Clap;
use std::{collections::HashMap, net::IpAddr, path::PathBuf};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum OptsValidateError {
    #[error("invalid path mapping, less client paths than server paths")]
    InvalidPathMappingLessClientPaths(),

    #[error("invalid path mapping, less server paths than server paths")]
    InvalidPathMappingLessServerPaths(),
}

impl Opts {
    pub fn validate(self) -> Result<Opts, OptsValidateError> {
        if self.client_paths.len() < self.server_paths.len() {
            return Err(OptsValidateError::InvalidPathMappingLessClientPaths());
        }

        if self.client_paths.len() > self.server_paths.len() {
            return Err(OptsValidateError::InvalidPathMappingLessServerPaths());
        }

        Ok(self)
    }
}

impl From<Opts> for BsConfig {
    fn from(opts: Opts) -> Self {
        let mut path_mappings = HashMap::new();
        for (client_path, server_path) in opts.client_paths.into_iter().zip(opts.server_paths) {
            path_mappings.insert(client_path, server_path);
        }

        BsConfig::new(
            opts.client_ip,
            opts.client_user,
            opts.server_ip,
            opts.server_user,
            path_mappings,
        )
    }
}
