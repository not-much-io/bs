mod opts;

use bs::config::BsConfig;
use clap::Clap;
use opts::Opts;

fn main() {
    let _config: BsConfig = Opts::parse().into();
}
