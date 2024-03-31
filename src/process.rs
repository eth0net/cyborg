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
            self.process_dir(path)?;
        } else if path.is_file() {
            self.process_file(path)?;
        } else {
            log::warn!("{} is not a file or directory", path.display());
            bail!("not a file or directory");
        }

        log::debug!("processed path: {}", path.display());

        Ok(())
    }

    fn process_dir(&self, dir: &Path) -> anyhow::Result<()> {
        log::debug!("processing dir: {}", dir.display());

        for entry in dir.read_dir()? {
            let path = entry?.path();
            self.process_path(path.as_path())?;
        }

        log::debug!("processed dir: {}", dir.display());

        Ok(())
    }

    fn process_file(&self, file: &Path) -> anyhow::Result<()> {
        log::debug!("processing file: {}", file.display());

        let name = file.file_name().with_context(|| "getting file name")?;
        let name = name.to_str().with_context(|| "converting name to str")?;

        log::trace!("old name: {}", name);

        let name: Comic = name.parse()?;

        log::trace!("new name: {}", name);

        log::debug!("processed file: {}", name);

        Ok(())
    }
}
