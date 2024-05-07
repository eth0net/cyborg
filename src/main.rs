use clap::Parser;
use indicatif::MultiProgress;

use cyborg::command::Args;
use cyborg::log;
use cyborg::organise::{Organiser, Settings};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let multibar = MultiProgress::new();

    log::init(&args, multibar.clone())?;

    let settings = Settings::from_args(&args);

    let organiser = Organiser::new(settings, multibar);

    organiser.organise(args.paths)
}
