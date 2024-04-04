use std::fs;
use std::path::Path;

use anyhow::{bail, Context};

use crate::args::Args;
use crate::comic::Comic;

// todo: rename to Organiser

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
            let result = path.metadata();

            if let Err(err) = result {
                let message = format!("failed to get metadata for: {}", path.display());
                match self.args.exit {
                    true => return Err(err).context(message),
                    false => {
                        log::error!("{message}: {err:#}");
                        continue;
                    }
                }
            }

            log::trace!("got metadata for: {}", path.display());
            let meta = result.unwrap();

            let result = match meta.is_dir() {
                true => self.process_dir(path),
                false => self.process_file(path),
            };

            if let Err(err) = result {
                let message = format!("failed to process target: {}", path.display());
                match self.args.exit {
                    true => return Err(err).context(message),
                    false => {
                        log::error!("{message}: {err:#}");
                        continue;
                    }
                }
            }
        }

        log::trace!("processed targets");

        Ok(())
    }

    fn process_dir(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing dir: {}", path.display());

        let directory = path
            .read_dir()
            .with_context(|| format!("failed to read directory: {}", path.display()))?;

        log::trace!("read dir: {}", path.display());

        for entry in directory {
            if let Err(err) = entry {
                let message = format!("failed to read directory entry: {}", path.display());
                match self.args.exit {
                    true => return Err(err).context(message),
                    false => {
                        log::error!("{message}: {err:#}");
                        continue;
                    }
                }
            }

            let path = &entry.unwrap().path();

            let result = path.metadata();

            if let Err(err) = result {
                let message = format!("failed to get metadata for: {}", path.display());
                match self.args.exit {
                    true => return Err(err).context(message),
                    false => {
                        log::error!("{message}: {err:#}");
                        continue;
                    }
                }
            }

            log::trace!("got metadata for: {}", path.display());
            let meta = result.unwrap();

            let result = match [meta.is_dir(), self.args.recursive] {
                [true, true] => self.process_dir(path),
                [true, false] => {
                    log::trace!("skipping subdirectory: {}", path.display());
                    continue;
                }
                [false, _] => self.process_file(path),
            };

            if let Err(err) = result {
                let message = format!("failed to process directory entry: {}", path.display());
                match self.args.exit {
                    true => return Err(err).context(message),
                    false => {
                        log::error!("{message}: {err:#}");
                        continue;
                    }
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

        match [self.args.dry_run, self.args.move_files] {
            [true, true] => {
                log::info!("would move: {} -> {}", path.display(), new_path.display());
            }
            [true, false] => {
                log::info!("would copy: {} -> {}", path.display(), new_path.display());
            }
            [false, true] => {
                log::info!("moving: {} -> {}", path.display(), new_path.display());
                fs::rename(path, new_path).context("moving file")?;
            }
            [false, false] => {
                log::info!("copying: {} -> {}", path.display(), new_path.display());
                fs::copy(path, new_path).context("copying file")?;
            }
        }

        log::debug!("processed file: {}", path.display());

        Ok(())
    }
}
