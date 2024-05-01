use log::LevelFilter;

use crate::command::Args;

/// Initialize the logger.
pub fn init(args: &Args) {
    let mut level = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    if args.dry_run {
        level = level.clamp(LevelFilter::Info, LevelFilter::Trace);
    }

    if args.quiet {
        level = LevelFilter::Off;
    }

    env_logger::builder().filter_level(level).init();
}
