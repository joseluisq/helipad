#![deny(warnings)]

extern crate executor;

use executor::{App, Config, Result};
use structopt::StructOpt;
// use serde_json::json;

fn main() -> Result {
    let opts = Config::from_args();

    let cmds = &[
        "echo abc",
        "for i in {1..10}; do echo $i",
        "sleep 0.05 && echo 000",
        "done",
        "echo xyz",
        "for i in {1..10}; do echo $i",
        "sleep 0.05 && echo 111",
        "done",
    ];

    App::new(opts).run(cmds)?;

    Ok(())
}
