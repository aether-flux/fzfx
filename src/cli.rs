use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "fzfx", author, version, about = "Fast semantic search")]
pub struct Args {
    #[arg(short, long)]
    pub query: Option<String>,

    #[arg(short = 'f', long = "data-file")]
    pub data_file: Option<PathBuf>,

    #[arg(
        short = 'k',
        long = "top-k",
        default_value_t = 10,
        help = "Number of matches to display (default = 10)"
    )]
    pub top_k: usize,

    #[arg(short, long)]
    pub raw: bool,

    #[arg(short, long)]
    pub copy: bool,

    #[arg(short = 'x', long)]
    pub exec: bool,

    #[arg(short = 'C', long)]
    pub clear_cache: bool,

    #[arg(long)]
    pub clear_all: bool,
}
