use clap::Parser;

use cyborg::args::Args;
use cyborg::logger;
use cyborg::process::{Processor, ProcessorSettings};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    logger::init(&args);

    let settings = ProcessorSettings::from_args(&args);

    let processor = Processor::new(settings);

    processor.process(args.targets)
}
