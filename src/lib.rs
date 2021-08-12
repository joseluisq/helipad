#![deny(warnings)]

// #[macro_use]
extern crate anyhow;

#[macro_use]
pub mod result;
pub mod exec;
pub mod shell;

pub use crate::result::*;
