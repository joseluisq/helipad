use duct::{cmd, ReaderHandle};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;
use std::path::PathBuf;

use crate::shell;
use crate::Result;

pub struct Exec {}

pub struct Step {
    pub workdir: PathBuf,
    pub envs: HashMap<String, String>,
}

impl Exec {
    pub fn new() -> Self {
        Exec {}
    }

    pub fn run(&self, step: Step, args: &[&str]) -> Result<Option<Lines<BufReader<ReaderHandle>>>> {
        if args.is_empty() {
            return Ok(None);
        }

        let mut cmds = cmd!(
            shell::BIN,
            shell::ARGS,
            format!("{} {}", shell::DEFAULTS, args.join(";"))
        )
        .dir(step.workdir);

        if !step.envs.is_empty() {
            cmds = cmds.full_env(step.envs)
        }

        let reader = cmds.stderr_to_stdout().reader()?;

        Ok(Some(BufReader::new(reader).lines()))
    }
}

impl Default for Exec {
    fn default() -> Self {
        Self::new()
    }
}
