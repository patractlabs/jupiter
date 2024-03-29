//! Jupiter CLI library.

// #![warn(missing_docs)]
#![warn(unused_extern_crates)]

/// jupiter para chain configurations.
pub mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

pub use command::*;
pub use sc_cli::Result;
