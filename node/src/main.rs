//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
mod members_config;
#[macro_use]
mod service;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
	command::run()
}
