use crate::{Config, Exec, Result, Step};
use std::collections::HashMap;

pub struct Helipad {
    opts: Config,
}

impl Helipad {
    pub fn new(opts: Config) -> Self {
        Self { opts }
    }

    // TODO: execute steps instead of commands
    pub fn exec(&self, cmds: &[&str]) -> Result {
        // TODO: read and use options
        let workdir = self.opts.workdir.to_owned();

        // TODO: append custom envs
        let mut envs: HashMap<_, _> = std::env::vars().collect();
        envs.insert("FOO".to_owned(), "bar".to_owned());

        // TODO: process steps instead of commands
        let step = Step::new(workdir, envs);
        let exc = Exec::new();

        // TODO: use a closure and pass `res` into it
        if let Some(lines) = exc.run(step, cmds)? {
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
}
