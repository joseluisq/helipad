#![deny(warnings)]

extern crate executor;

use std::collections::HashMap;

use executor::exec::{Exec, Step};
use executor::Result;
use serde_json::json;

fn main() -> Result {
    let ex = Exec::new();

    let mut envs: HashMap<_, _> = std::env::vars().collect();
    envs.insert("FOO".to_owned(), "bar".to_owned());

    let step = Step {
        workdir: std::env::current_dir()?,
        envs,
    };

    let args = &[
        "echo abc",
        "for i in {1..10}",
        "do echo $i",
        "sleep 0.05",
        "done",
    ];

    if let Some(lines) = ex.run(step, args)? {
        let mut res = vec![];
        let mut n = 0_usize;
        for line in lines {
            let line = line?;
            n += 1;
            println!(r#"{} {}"#, n, line);
            res.push(line);
        }

        println!();
        println!("JSON:");
        println!("{}", json!(res));
    }

    Ok(())
}
