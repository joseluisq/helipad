#![deny(warnings)]

extern crate executor;

use duct::cmd;
use executor::Result;
use std::io::prelude::*;
use std::io::BufReader;
use serde_json::json;

fn main() -> Result {
    // just an example
    let cmds = cmd!(
        "bash",
        "-c",
        // Fail:
        // "set -ex; echo abc; for i in {1..10}; do echo $i; sleep 0.05; xyz; done",
        // Ok:
        "set -ex; echo abc; for i in {1..10}; do echo $i; sleep 0.05; done",
    );

    let reader = cmds.stderr_to_stdout().reader()?;
    let lines = BufReader::new(reader).lines();

    let mut v = vec![];
    let mut n = 0_u64;
    for line in lines.into_iter() {
        let s = line?;
        n = n+1;
        println!(r#"{} {}"#, n, s);
        v.push(s);
    }

    println!();
    println!("JSON:");
    println!("{}", json!(v));

    Ok(())
}
