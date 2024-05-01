use std::path::PathBuf;

use crate::args::Args;

#[derive(Default)]
/// Settings for the processor
pub struct ProcessorSettings {
    /// The output directory for the processed files
    pub output: PathBuf,
    /// Whether to output files in series subdirectories
    pub series: bool,
    /// Whether to move files instead of copying them
    pub move_files: bool,
    /// Whether to perform a dry run
    pub dry_run: bool,
    /// Whether to exit after processing
    pub exit: bool,
    /// Whether to force overwrite of existing files
    pub force: bool,
    /// Whether to process files recursively
    pub recursive: bool,
}

impl ProcessorSettings {
    /// Create a new ProcessorSettings instance with default values
    pub fn new() -> ProcessorSettings {
        ProcessorSettings::default()
    }

    /// Create a new ProcessorSettings instance from the provided Args
    pub fn from_args(args: &Args) -> ProcessorSettings {
        ProcessorSettings {
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
