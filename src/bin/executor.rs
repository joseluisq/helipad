#![deny(warnings)]

extern crate executor;

use std::collections::HashMap;

use executor::{Exec, Result, Step};
// use serde_json::json;

fn main() -> Result {
    let workdir = std::env::current_dir()?;

    let mut envs: HashMap<_, _> = std::env::vars().collect();
    envs.insert("FOO".to_owned(), "bar".to_owned());

    let step = Step::new(workdir, envs);

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

    if let Some(lines) = Exec::new().run(step, cmds)? {
        let mut res = vec![];
        let mut n = 0_usize;
        for line in lines {
            let line = line?;
            n += 1;
            println!(r#"{} {}"#, n, line);
            res.push(line);
        }

        // println!();
        // println!("JSON:");
        // println!("{}", json!(res));
    }

    Ok(())
}
