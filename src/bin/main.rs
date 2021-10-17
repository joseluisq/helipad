#![deny(warnings)]
#![deny(rust_2018_idioms)]

use helipad::{Config, Helipad, Result};
use structopt::StructOpt;

fn main() -> Result {
    Helipad::new(Config::from_args()).start()?;
    Ok(())
}
