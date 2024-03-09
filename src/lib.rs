use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use comic::Comic;

pub mod comic;

#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Args {
    /// The input files to be organised.
    ///
    /// If a directory is provided, all files in the directory
    /// will be processed.
    pub targets: Vec<PathBuf>,

    /// The output directory.
    ///
    /// If not provided, files will be written alongside the originals.
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Whether to process directories recursively.
    ///
    /// If not provided, only the top-level files will be processed.
    #[arg(short, long)]
    pub recursive: bool,
    // fast_fail: bool,
    // verbose: bool,
    // quiet: bool,
    // dry_run: bool,
    // force: bool,
    // config: Option<PathBuf>,
}

pub fn process_path(path: PathBuf, recursive: bool) -> anyhow::Result<()> {
    log::info!("processing path: {}", path.display());
    if path.is_dir() {
        process_dir(path, recursive)?;
    } else if path.is_file() {
        process_file(path)?;
    } else {
        log::warn!("{} is not a file or directory", path.display());
    }

    Ok(())
}

pub fn process_dir(dir: PathBuf, recursive: bool) -> anyhow::Result<()> {
    log::info!("processing dir: {}", dir.display());
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        process_path(path, recursive)?;
    }

    Ok(())
}

pub fn process_file(file: PathBuf) -> anyhow::Result<()> {
    log::info!("processing file: {}", file.display());

    let name = file.file_name().with_context(|| "getting file name")?;

    let new_name: Comic = name.to_str().unwrap().parse()?;

    log::info!("new name: {}", new_name);

    Ok(())
}
