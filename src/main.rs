use anyhow::Result;
use clap::Parser;

use crate::{cli::Args, handle_commands::handle_args};

mod cache;
mod cli;
mod context;
mod handle_commands;
mod util;

fn main() -> Result<()> {
    let args = Args::parse();
    handle_args(&args)?;

    Ok(())
}
