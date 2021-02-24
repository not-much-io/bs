mod opts;

use anyhow::Result;
use bs::{
    config::BsConfig,
    storage::{impls::cli_rsync::CLIRSyncStorageSyncer, StorageSyncer},
};
use clap::Clap;
use opts::Opts;

fn main() -> Result<()> {
    let config: BsConfig = Opts::parse().validate()?.into();
    let _result = CLIRSyncStorageSyncer::new(config.into()).sync();
    Ok(())
}
