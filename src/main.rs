use clap::Parser;

use cyborg::args::Args;
use cyborg::process::Processor;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();
    let processor = Processor::with_args(args.clone());

    processor.process()
}
