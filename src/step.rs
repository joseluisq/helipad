use std::collections::HashMap;
use std::path::PathBuf;

/// Defines a pipeline step.
pub struct Step {
    pub workdir: PathBuf,
    pub envs: HashMap<String, String>,
}

impl Step {
    pub fn new(workdir: PathBuf, envs: HashMap<String, String>) -> Self {
        Self { workdir, envs }
    }
}
