#![deny(warnings)]
#![deny(rust_2018_idioms)]

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

#[macro_use]
pub mod result;
pub mod app;
pub mod config;
pub mod exec;
pub mod helpers;
pub mod logger;
pub mod manifest;
pub mod pipelines;
pub mod shell;
pub mod step;

pub use crate::app::*;
pub use crate::config::*;
pub use crate::exec::*;
pub use crate::result::*;
pub use crate::step::*;
