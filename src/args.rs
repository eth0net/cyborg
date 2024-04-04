use clap::{ArgAction, Parser};
use std::path::PathBuf;

// todo: rename to Organiser
// todo: option to clobber existing destination files
// todo: option to copy files instead of moving them
// todo: option to clean up empty source directories
// todo: add support for configuration files

#[derive(Clone, Debug, Default, Parser)]
#[command(version, author, about)]
#[group(id = "noisy", multiple = true)]
pub struct Args {
    /// A list of files or directories to process.
    ///
    /// For a directory, each direct child file will be processed.
    pub targets: Vec<PathBuf>,

    /// Output directory for organised files.
    ///
    /// If the directory does not exist, it will be created.
    ///
    /// If not provided, the current directory will be used.
    #[arg(short, long, default_value = ".", value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Organise files into subdirectories by series.
    ///
    /// If not provided, files will be placed in the output directory.
    #[arg(short, long)]
    pub series: bool,

    /// Print steps without making changes (implies -vv).
    ///
    /// If not provided, changes will be made to the filesystem.
    #[arg(short, long, group = "noisy")]
    pub dry_run: bool,

    /// Stop processing after the first error.
    ///
    /// If not provided, errors will be logged and processing will continue.
    #[arg(short, long, default_value = "false")]
    pub exit: bool,

    /// Recursively process files in subdirectories.
    ///
    /// If not provided, only the top-level files will be processed.
    #[arg(short, long)]
    pub recursive: bool,

    /// Increase verbosity (can be used multiple times).
    ///
    /// If not provided, only errors will be logged.
    #[arg(short, long, action = ArgAction::Count, group = "noisy")]
    pub verbose: u8,

    /// Suppress all output (conflicts with verbose).
    ///
    /// If not provided, output will be printed as normal.
    #[arg(short, long, conflicts_with = "noisy")]
    pub quiet: bool,
}
