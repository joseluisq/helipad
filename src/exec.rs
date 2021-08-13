use duct::{cmd, ReaderHandle};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;

use crate::{shell, step::Step, Result};

pub struct Exec {}

impl Exec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, step: Step, cmds: &[&str]) -> Result<Option<Lines<BufReader<ReaderHandle>>>> {
        if cmds.is_empty() {
            return Ok(None);
        }

        let mut cmds = cmd!(
            shell::BIN,
            shell::ARGS,
            format!("{} {}", shell::DEFAULTS, cmds.join(";"))
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
