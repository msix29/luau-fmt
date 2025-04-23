mod cli;

use clap::Parser;
use cli::format_path;
use luau_formatter::load_config;
use std::{io, path::PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version = VERSION)]
struct Cli {
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    format_path(&args.path, &load_config(&args.path).unwrap_or_default())
}
