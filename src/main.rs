mod cli;

use clap::Parser;
use cli::format_path;
use luau_formatter::Config;
use std::{io, path::{Path, PathBuf}};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version = VERSION)]
struct Cli {
    path: PathBuf,
}

fn load_config<P: AsRef<Path>>(path: P) -> Config {
    Config::default()
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    format_path(&args.path, &load_config(&args.path))
}
