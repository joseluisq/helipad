use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::helpers;
use crate::result::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PipelineKind {
    Host,
    Docker,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformOs {
    Linux,
    Macos,
    Freebsd,
    Windows,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformArch {
    Amd64,
    Arm64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PipelinePlatform {
    pub os: PlatformOs,
    pub arch: PlatformArch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Holds the env value or script.
pub enum EnvValue {
    /// The value as string
    Value(String),
    /// The value as boolean
    Boolean(bool),
    /// The value as number
    Number(isize),
    /// The value as a list of strings
    List(Vec<String>),
    /// The value as a list of numbers
    ListInt(Vec<isize>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Script value
pub enum ScriptValue {
    /// The script text as single line
    SingleLine(String),
    /// The script text lines
    Text(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct PipelineStep {
    pub name: String,
    pub env: Option<HashMap<String, EnvValue>>,
    pub script: Option<ScriptValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pipeline {
    // General
    pub kind: PipelineKind,
    pub name: String,

    // Platform
    pub platform: PipelinePlatform,

    // Steps
    #[serde(rename(deserialize = "step"))]
    pub steps: Option<Vec<PipelineStep>>,
}

/// Read a TOML file from path.
pub fn read_file(path: &Path) -> Result<toml::Value> {
    let toml_str = helpers::read(path).with_context(|| {
        format!(
            "error trying to deserialize pipeline \"{}\" file toml.",
            path.display()
        )
    })?;

    let first_error = match toml_str.parse() {
        Ok(res) => return Ok(res),
        Err(err) => err,
    };

    let mut second_parser = toml::de::Deserializer::new(&toml_str);
    second_parser.set_require_newline_after_table(false);
    if let Ok(res) = toml::Value::deserialize(&mut second_parser) {
        let msg = format!(
            "\
TOML file found which contains invalid syntax and will soon not parse
at `{}`.
The TOML spec requires newlines after table definitions (e.g., `[a] b = 1` is
invalid), but this file has a table header which does not have a newline after
it. A newline needs to be added and this warning will soon become a hard error
in the future.",
            path.display()
        );
        println!("{}", &msg);
        return Ok(res);
    }

    let first_error = anyhow::Error::from(first_error);
    Err(first_error.context("could not parse input as TOML format"))
}
