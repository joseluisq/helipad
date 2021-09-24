#![deny(warnings)]

extern crate helipad;

use helipad::{Config, Helipad, Result};
use structopt::StructOpt;
// use serde_json::json;

fn main() -> Result {
    let opts = Config::from_args();

    // TODO: read `.pipelines/pipeline.toml` file

    let cmds = &[
        "echo abc",
        "for i in {1..10}; do echo $i",
        "sleep 0.025 && echo 000",
        "done",
        "echo xyz",
        "for i in {1..10}; do echo $i",
        "sleep 0.025 && echo 111",
        "done",
    ];

    // TODO: process steps instead of commands
    Helipad::new(opts).exec(cmds)?;

    Ok(())
}
