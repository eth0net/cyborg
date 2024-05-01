use std::path::PathBuf;

use crate::args::Args;

#[derive(Default)]
pub struct ProcessorSettings {
    pub output: PathBuf,
    pub series: bool,
    pub move_files: bool,
    pub dry_run: bool,
    pub exit: bool,
    pub force: bool,
    pub recursive: bool,
}

impl ProcessorSettings {
    pub fn new() -> ProcessorSettings {
        ProcessorSettings::default()
    }

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
