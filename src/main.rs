use std::error::Error;
use std::path::PathBuf;

use clap::Parser;

use cyborg::comic::Comic;

#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Args {
    /// The input files to be organised.
    ///
    /// If a directory is provided, all files in the directory
    /// will be processed.
    targets: Vec<PathBuf>,

    /// The output directory.
    ///
    /// If not provided, files will be written alongside the originals.
    #[arg(short, long, value_name = "DIR")]
    output: Option<PathBuf>,

    /// Whether to process directories recursively.
    ///
    /// If not provided, only the top-level files will be processed.
    #[arg(short, long)]
    recursive: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    for path in args.targets {
        process_path(path, args.recursive)?;
    }

    Ok(())
}

fn process_path(path: PathBuf, recursive: bool) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        process_dir(path, recursive)?;
    } else if path.is_file() {
        process_file(path)?;
    } else {
        println!("{} is not a file or directory", path.display());
    }

    Ok(())
}

fn process_dir(dir: PathBuf, recursive: bool) -> Result<(), Box<dyn Error>> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        process_path(path, recursive)?;
    }

    Ok(())
}

fn process_file(file: PathBuf) -> Result<(), Box<dyn Error>> {
    let name = file.file_name().ok_or("No file name")?;

    let new_name: Comic = name.to_str().unwrap().parse()?;

    println!("{:?} => {}", name, new_name);

    Ok(())
}
