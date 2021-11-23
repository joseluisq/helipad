#[cfg(not(windows))]
pub const BIN: &str = "/bin/sh";
#[cfg(not(windows))]
pub const ARGS: &str = "-c";
#[cfg(not(windows))]
pub const DEFAULTS: &str = "set -ex;";

// TODO: Windows powershell
