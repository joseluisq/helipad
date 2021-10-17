use std::{collections::HashMap, path::Path};

use crate::{
    manifest::{EnvValue, Pipeline, ScriptValue},
    Exec, Result, Step,
};

pub fn run(pipeline: &Pipeline, workdir: &Path) -> Result {
    // Iterate over pipeline steps
    match &pipeline.steps {
        Some(steps) => {
            println!(r#"Executing pipeline: {}"#, &pipeline.name);

            for step in steps {
                // TODO: Validate step name
                if step.name.is_empty() {
                    bail!("one step has an empty name.")
                }

                // Append custom manifest envs
                let mut envs: HashMap<_, _> = std::env::vars().collect();
                if let Some(vars) = &step.env {
                    for (k, v) in vars {
                        match v {
                            EnvValue::Value(s) => envs.insert(k.to_owned(), s.to_owned()),
                            EnvValue::Boolean(b) => envs.insert(k.to_owned(), b.to_string()),
                            EnvValue::Number(n) => envs.insert(k.to_owned(), n.to_string()),
                            EnvValue::List(l) => envs.insert(k.to_owned(), l.join(",")),
                            EnvValue::ListInt(l) => envs.insert(
                                k.to_owned(),
                                l.iter()
                                    .map(ToString::to_string)
                                    .collect::<Vec<_>>()
                                    .join(","),
                            ),
                        };
                    }
                };

                // Parse `script` with its possible values
                let cmds = match &step.script {
                    Some(s) => match s {
                        ScriptValue::SingleLine(s) => vec![s.to_owned()],
                        ScriptValue::Text(s) => s.to_owned(),
                    },
                    None => vec![],
                };

                println!(r#"Executing step: {}"#, &step.name);

                let stepv = Step::new(workdir.to_path_buf(), envs);
                let exc = Exec::new();

                // TODO: use a closure and pass `res` into it
                if let Some(lines) = exc.run(stepv, &cmds)? {
                    let mut res = vec![];
                    let mut n = 0_usize;
                    for line in lines {
                        let line = line?;
                        n += 1;
                        println!(r#"{} {}"#, n, line);
                        res.push(line);
                    }

                    // TODO: JSON support
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
