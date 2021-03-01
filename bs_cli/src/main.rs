mod opts;

use anyhow::Result;
use bs::{
    config::BsConfig,
    storage::{impls::cli_rsync::CliRSyncStorageSyncer, StorageSyncer},
};
use clap::Clap;
use opts::Opts;

fn main() -> Result<()> {
    let config: BsConfig = Opts::parse().validate()?.into();
    let _result = CliRSyncStorageSyncer::new(config.into()).sync();
    Ok(())
}
