mod conf;
mod opts;

use anyhow::Result;
use clap::Clap;
use conf::DistRunnerConfig;
use distrunner::DistRunner;
use opts::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let config = DistRunnerConfig::new(opts.config);
    let distrunner: DistRunner = config.into();

    distrunner.run().await
}
