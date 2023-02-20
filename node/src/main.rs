//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod client;
mod command;
mod rpc;
mod tracing;

fn main() -> sc_cli::Result<()> {
	command::run()
}
