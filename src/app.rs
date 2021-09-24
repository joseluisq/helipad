use crate::{Config, Exec, Result, Step};
use std::collections::HashMap;

pub struct App {
    opts: Config,
}

impl App {
    pub fn new(opts: Config) -> Self {
        Self { opts }
    }

    pub fn run(&self, cmds: &[&str]) -> Result {
        // TODO: read and use options
        let workdir = self.opts.workdir.to_owned();

        // TODO: append custom envs
        let mut envs: HashMap<_, _> = std::env::vars().collect();
        envs.insert("FOO".to_owned(), "bar".to_owned());

        let step = Step::new(workdir, envs);

        // let cmds = &[
        //     "echo abc",
        //     "for i in {1..10}; do echo $i",
        //     "sleep 0.05 && echo 000",
        //     "done",
        //     "echo xyz",
        //     "for i in {1..10}; do echo $i",
        //     "sleep 0.05 && echo 111",
        //     "done",
        // ];

        // TODO:
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
}
