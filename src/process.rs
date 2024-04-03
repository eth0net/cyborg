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
        log::trace!("processing targets");

        for path in &self.args.targets {
            let meta = path.metadata().context("getting metadata")?;

            let result = match meta.is_dir() {
                true => self.process_dir(path),
                false => self.process_file(path),
            };

            if let Err(err) = result {
                let context = format!("failed to process: {}", path.display());

                log::error!("{context}: {err}");

                if self.args.fail_fast {
                    log::trace!("failing fast");
                    return Err(err).context(context);
                }
            }
        }

        log::trace!("processed targets");

        Ok(())
    }

    fn process_dir(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing dir: {}", path.display());

        for entry in path.read_dir()? {
            let path = &entry?.path();

            let meta = path.metadata().context("getting metadata")?;

            let result = match [meta.is_dir(), self.args.recursive] {
                [true, true] => self.process_dir(path),
                [true, false] => {
                    log::trace!("skipping dir: {}", path.display());
                    continue;
                }
                [false, _] => self.process_file(path),
            };

            if let Err(err) = result {
                let context = format!("failed to process: {}", path.display());

                log::error!("{context}: {err}");

                if self.args.fail_fast {
                    log::trace!("failing fast");
                    return Err(err).context(context);
                }
            }
        }

        log::debug!("processed dir: {}", path.display());

        Ok(())
    }

    fn process_file(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing file: {}", path.display());

        let name = path.file_name().context("getting file name")?;
        let name = name.to_str().context("converting name to str")?;

        log::trace!("old name: {}", name);

        let comic: Comic = name.parse()?;
        let new_name = comic.to_string();

        log::trace!("new name: {}", &new_name);

        let output_dir = match &self.args.output {
            Some(output) => output,
            None => path.parent().context("getting parent dir")?,
        };

        if !output_dir.is_dir() {
            log::error!("output dir is not a directory: {}", output_dir.display());
            bail!("output dir is not a directory");
        }

        let output_dir = match self.args.series {
            true => output_dir.join(comic.series),
            false => output_dir.to_path_buf(),
        };

        log::trace!("output dir: {}", output_dir.display());

        match [output_dir.exists(), self.args.dry_run] {
            [false, false] => {
                log::info!("creating output dir: {}", output_dir.display());
                fs::create_dir_all(&output_dir).context("creating output dir")?;
            }
            [false, true] => log::info!("would create output dir: {}", output_dir.display()),
            _ => log::trace!("output dir exists: {}", output_dir.display()),
        }

        let new_path = output_dir.join(new_name);

        log::trace!("new path: {}", new_path.display());

        match !self.args.dry_run {
            true => {
                log::info!("renaming: {} -> {}", path.display(), new_path.display());
                fs::rename(path, new_path).context("renaming file")?;
            }
            false => {
                log::info!("would rename: {} -> {}", path.display(), new_path.display());
            }
        }

        log::debug!("processed file: {}", path.display());

        Ok(())
    }
}
