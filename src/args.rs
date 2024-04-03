use clap::{ArgAction, Parser};
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

    /// Split series into separate directories in the output directory.
    ///
    /// If not provided, no directories will be created.
    #[arg(short, long)]
    pub split_series: bool,

    /// Whether to perform actions, or just print steps to be taken. Implies -vv.
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

    /// Increase verbosity, can be used multiple times.
    ///
    /// If not provided, only errors will be logged.
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    /// Whether to suppress all output. This will override --verbose.
    ///
    /// If not provided, output will be printed as normal.
    #[arg(short, long)]
    pub quiet: bool,
    // force: bool,
    // config: Option<PathBuf>,
}
