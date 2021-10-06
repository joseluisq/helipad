use indexmap::IndexMap;
use serde::Deserialize;
use std::path::Path;

use crate::helpers;
use crate::result::Result;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct PipelinePlatform {
    pub os: Option<String>,
    pub arch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds the env value or script
pub enum EnvValue {
    /// The value as string
    Value(String),
    /// The value as boolean
    Boolean(bool),
    /// The value as number
    Number(isize),
    /// The value as a list of strings
    List(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Script value
pub enum ScriptValue {
    /// The script text as single line
    SingleLine(String),
    /// The script text lines
    Text(Vec<String>),
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct PipelineStep {
    pub name: String,
    pub env: Option<IndexMap<String, EnvValue>>,
    pub script: Option<ScriptValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PipelineKind {
    Docker,
    Host,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Pipeline {
    // General
    pub kind: PipelineKind,
    pub name: String,

    // Platform
    pub platform: Option<PipelinePlatform>,

    // Steps
    #[serde(rename(deserialize = "step"))]
    pub steps: Option<Vec<PipelineStep>>,
}

pub fn read_file(path: &Path) -> Result<toml::Value> {
    let toml_str = helpers::read(path)?;

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
    Err(first_error.context("could not parse input as TOML"))
}
