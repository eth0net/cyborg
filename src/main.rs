use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Args {
    /// The input files to be organised.
    ///
    /// If a directory is provided, all files in the directory
    /// will be processed.
    targets: Vec<PathBuf>,

    /// The output directory.
    ///
    /// If not provided, files will be written alongside the originals.
    #[arg(short, long, value_name = "DIR")]
    output: Option<PathBuf>,

    /// Whether to process directories recursively.
    ///
    /// If not provided, only the top-level files will be processed.
    #[arg(short, long)]
    recursive: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    println!("Hello, comics!");
}
