use std::path::PathBuf;

use crate::command::Args;

#[derive(Default)]
/// Settings for the organiser
pub struct Settings {
    /// The output directory for the organised files
    pub output: PathBuf,
    /// Whether to output files in series subdirectories
    pub series: bool,
    /// Whether to move files instead of copying them
    pub move_files: bool,
    /// Whether to perform a dry run
    pub dry_run: bool,
    /// Whether to exit after organising
    pub exit: bool,
    /// Whether to force overwrite of existing files
    pub force: bool,
    /// Whether to organise files recursively
    pub recursive: bool,
}

impl Settings {
    /// Create a new OrganiserSettings instance with default values
    pub fn new() -> Settings {
        Settings::default()
    }

    /// Create a new OrganiserSettings instance from the provided Args
    pub fn from_args(args: &Args) -> Settings {
        Settings {
            output: args.output.clone(),
            series: args.series,
            move_files: args.move_files,
            dry_run: args.dry_run,
            exit: args.exit,
            force: args.force,
            recursive: args.recursive,
        }
    }
}
