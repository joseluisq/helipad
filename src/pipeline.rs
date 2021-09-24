use serde::Deserialize;
use std::path::Path;

use crate::helpers;
use crate::result::Result;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TomlPlatform {
    pub os: Option<String>,
    pub arch: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TomlPipeline {
    pub kind: String,
    pub name: String,
    // Platform
    pub platform: Option<TomlPlatform>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlManifest {
    pub pipeline: Option<Box<TomlPipeline>>,
    pub platform: Option<Box<TomlPipeline>>,
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
