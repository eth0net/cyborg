use clap::Parser;

use cyborg::process::Processor;
use cyborg::Args;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();
    let processor = Processor::with_args(args.clone());

    processor.process()
}
