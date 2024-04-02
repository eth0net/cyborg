use std::fs;
use std::path::Path;

use anyhow::{bail, Context};

use crate::args::Args;
use crate::comic::Comic;

pub struct Processor {
    args: Args,
}

impl Processor {
    pub fn with_args(args: Args) -> Processor {
        Processor { args }
    }

    pub fn process(&self) -> anyhow::Result<()> {
        for path in &self.args.targets {
            if let Err(err) = self.process_path(path.as_path()) {
                log::error!("failed to process path: {}: {}", path.display(), err);
                match self.args.fail_fast {
                    true => bail!("processing path: {}: {}", path.display(), err),
                    false => continue,
                }
            }
        }

        Ok(())
    }

    fn process_path(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing path: {}", path.display());

        if path.is_dir() {
            log::trace!("path is a directory");
            self.process_dir(path)?;
        } else if path.is_file() {
            log::trace!("path is a file");
            self.process_file(path)?;
        } else {
            log::warn!("{} is not a file or directory", path.display());
            bail!("not a file or directory");
        }

        log::debug!("processed path: {}", path.display());

        Ok(())
    }

    fn process_dir(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing dir: {}", path.display());

        for entry in path.read_dir()? {
            let path = entry?.path();
            self.process_path(path.as_path())?;
        }

        log::debug!("processed dir: {}", path.display());

        Ok(())
    }

    fn process_file(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing file: {}", path.display());

        let name = path.file_name().with_context(|| "getting file name")?;
        let name = name.to_str().with_context(|| "converting name to str")?;

        log::trace!("old name: {}", name);

        let comic: Comic = name.parse()?;
        let new_name = comic.to_string();

        log::trace!("new name: {}", &new_name);

        let new_path = match &self.args.output {
            Some(output) => output.join(&new_name),
            None => path.with_file_name(&new_name),
        };

        log::trace!("new path: {}", new_path.display());

        match !self.args.dry_run {
            true => {
                log::info!("renaming: {} -> {}", path.display(), new_path.display());
                fs::rename(path, new_path).with_context(|| "renaming file")?;
            }
            false => {
                log::info!("would rename: {} -> {}", path.display(), new_path.display());
            }
        }

        log::debug!("processed file: {}", path.display());

        Ok(())
    }
}
