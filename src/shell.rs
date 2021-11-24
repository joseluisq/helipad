// Unix Bash
#[cfg(not(windows))]
pub const BIN: &str = "/bin/sh";
#[cfg(not(windows))]
pub const ARGS: &str = "-c";
#[cfg(not(windows))]
pub const DEFAULTS_START: &str = "set -ex;";
#[cfg(not(windows))]
pub const DEFAULTS_END: &str = "";

// Windows Powershell
#[cfg(windows)]
pub const BIN: &str = "powershell.exe";
#[cfg(windows)]
pub const ARGS: &str = "-NoProfile -NonInteractive -Command";
#[cfg(windows)]
pub const DEFAULTS_START: &str = r#"$ErrorActionPreference = "Stop";"#;
#[cfg(windows)]
pub const DEFAULTS_END: &str = r#"; if ($LastExitCode -gt 0) { exit $LastExitCode };"#;
