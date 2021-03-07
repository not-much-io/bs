use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Kristo Koert <kristo.koert@gmail.com>")]
pub struct Opts {
    #[clap(long, default_value = "distrunner.json")]
    pub config: PathBuf,
}
