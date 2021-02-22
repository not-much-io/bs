mod opts;

use bs::{
    config::BsConfig,
    storage::{impls::cli_rsync::CLIRSyncStorageSyncer, StorageSyncer},
};
use clap::Clap;
use opts::Opts;

fn main() {
    let config: BsConfig = Opts::parse().into();

    let result = CLIRSyncStorageSyncer::new(config.into()).sync();
    if result.is_err() {
        println!("err")
    } else {
        println!("ok")
    }
}
