#![deny(warnings)]

extern crate helipad;

use helipad::{Config, Helipad, Result};
use structopt::StructOpt;
// use serde_json::json;

fn main() -> Result {
    let opts = Config::from_args();

    // TODO: process steps instead of commands
    Helipad::new(opts).start()?;

    Ok(())
}
