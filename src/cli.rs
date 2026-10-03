use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "fzfx", author, version, about = "Fast semantic search")]
pub struct Args {
    #[arg(short, long)]
    pub query: Option<String>,

    #[arg(short = 'f', long = "data-file")]
    pub data_file: Option<PathBuf>,
}
