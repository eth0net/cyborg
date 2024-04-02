use clap::Parser;

use cyborg::args::Args;
use cyborg::logger;
use cyborg::process::Processor;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    logger::init(&args);

    let processor = Processor::with_args(args.clone());

    processor.process()
}
