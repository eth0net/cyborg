use std::path::PathBuf;

use clap::Parser;

pub mod comic;
pub mod process;

#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Args {
    /// The input files to be organised.
    ///
    /// If a directory is provided, all files in the directory
    /// will be processed.
    pub targets: Vec<PathBuf>,

    /// The output directory.
    ///
    /// If not provided, files will be written alongside the originals.
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Whether to process directories recursively.
    ///
    /// If not provided, only the top-level files will be processed.
    #[arg(short, long)]
    pub recursive: bool,
    // fast_fail: bool,
    // verbose: bool,
    // quiet: bool,
    // dry_run: bool,
    // force: bool,
    // config: Option<PathBuf>,
}
