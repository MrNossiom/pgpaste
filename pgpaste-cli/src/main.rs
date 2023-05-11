use clap::Parser;
use config::Config;

mod args;
mod commands;
mod config;

use crate::args::{Commands, PGPasteArgs};

fn main() -> eyre::Result<()> {
	let args = PGPasteArgs::parse();
	let config = Config::new(&args)?;

	match &args.command {
		Commands::Create(create_args) => commands::create(create_args, &config)?,
		Commands::Read(read_args) => commands::read(read_args, &config)?,
	}

	Ok(())
}
