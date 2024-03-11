use clap::Parser;

use cyborg::Args;

fn main() {
    env_logger::init();

    let args = Args::parse();

    for path in args.targets {
        if let Err(err) = cyborg::process::process_path(path.as_path(), args.recursive) {
            log::error!("failed to process path: {}: {}", path.display(), err);
            match args.fail_fast {
                true => return,
                false => continue,
            }
        }
    }
}
