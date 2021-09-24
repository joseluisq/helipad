use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
pub struct Config {
    #[structopt(long, short = "c", default_value = "./exec.toml", env = "EXEC_CONFIG")]
    /// Pipeline configuration directory or file
    pub config: String,

    #[structopt(long, short = "w", default_value = "./", env = "EXEC_WORKDIR")]
    /// Working directory
    pub workdir: PathBuf,
}
