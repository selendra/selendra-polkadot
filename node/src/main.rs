//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;
mod shell_upgrade;

fn main() -> sc_cli::Result<()> {
	command::run()
}
