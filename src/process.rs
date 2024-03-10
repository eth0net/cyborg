use std::path::Path;

use anyhow::{bail, Context};

use crate::comic::Comic;

pub fn process_path(path: &Path, recursive: bool) -> anyhow::Result<()> {
    log::debug!("processing path: {}", path.display());

    if path.is_dir() {
        process_dir(path, recursive)?;
    } else if path.is_file() {
        process_file(path)?;
    } else {
        log::warn!("{} is not a file or directory", path.display());
        bail!("not a file or directory");
    }

    log::debug!("processed path: {}", path.display());

    Ok(())
}

pub fn process_dir(dir: &Path, recursive: bool) -> anyhow::Result<()> {
    log::debug!("processing dir: {}", dir.display());

    for entry in dir.read_dir()? {
        let path = entry?.path();
        process_path(path.as_path(), recursive)?;
    }

    log::debug!("processed dir: {}", dir.display());

    Ok(())
}

pub fn process_file(file: &Path) -> anyhow::Result<()> {
    log::debug!("processing file: {}", file.display());

    let name = file.file_name().with_context(|| "getting file name")?;
    let name = name.to_str().with_context(|| "converting name to str")?;

    log::trace!("old name: {}", name);

    let name: Comic = name.parse()?;

    log::trace!("new name: {}", name);

    log::debug!("processed file: {}", name);

    Ok(())
}
