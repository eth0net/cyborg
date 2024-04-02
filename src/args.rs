use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Parser)]
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

    /// Whether to perform actions, or just print steps to be taken.
    ///
    /// If not provided, actions will be taken as normal.
    #[arg(short, long)]
    pub dry_run: bool,

    /// Whether to exit on first failure.
    ///
    /// If not provided, errors will be logged before continuing.
    #[arg(short, long)]
    pub fail_fast: bool,

    /// Whether to process directories recursively.
    ///
    /// If not provided, only the top-level files will be processed.
    #[arg(short, long)]
    pub recursive: bool,
    // verbose: bool,
    // quiet: bool,
    // force: bool,
    // config: Option<PathBuf>,
}