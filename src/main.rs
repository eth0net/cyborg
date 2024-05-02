use clap::Parser;

use cyborg::command::Args;
use cyborg::log;
use cyborg::organise::{Organiser, Settings};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    log::init(&args);

    let settings = Settings::from_args(&args);

    let processor = Organiser::new(settings);

    processor.process(args.paths)
}
