use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
pub struct Config {
    #[structopt(long, short = "c", default_value = "pipeline.toml", env = "EXEC_CONFIG")]
    /// Pipeline configuration directory or file path.
    pub config: PathBuf,
}
