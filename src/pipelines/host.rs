use std::{collections::HashMap, path::PathBuf};

use crate::{manifest::Pipeline, Exec, Result, Step};

pub fn run(pipeline: &Pipeline, workdir: PathBuf) -> Result {
    // Iterate over pipeline steps
    match &pipeline.steps {
        Some(steps) => {
            for step in steps {
                // TODO: Validate name
                if step.name.is_empty() {
                    bail!("No the current step has an empty name.")
                }

                // TODO: append custom manifest envs
                let mut envs: HashMap<_, _> = std::env::vars().collect();
                envs.insert("FOO".to_owned(), "bar".to_owned());

                // TODO: process steps instead of commands
                let stepr = Step::new(workdir.to_owned(), envs);
                let exc = Exec::new();

                let cmds = &step.script;
                println!(r#"Executing step: {}"#, &step.name);

                // TODO: use a closure and pass `res` into it
                if let Some(lines) = exc.run(stepr, &cmds)? {
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

                println!();
            }
        }
        None => {
            bail!("No steps found in the current pipeline file.")
        }
    }

    Ok(())
}
