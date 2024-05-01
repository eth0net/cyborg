use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::{comic::Meta, organise::Settings};

#[derive(Default)]
/// Processor for organising comic files
pub struct Organiser {
    /// Settings for the processor
    settings: Settings,
}

impl Organiser {
    /// Create a new Processor instance with the provided settings
    pub fn new(settings: Settings) -> Organiser {
        Self { settings }
    }

    /// Process the provided targets
    pub fn process(&self, targets: Vec<PathBuf>) -> anyhow::Result<()> {
        log::trace!("processing targets");

        for path in targets {
            let result = path.metadata();

            if let Err(err) = result {
                let message = format!("failed to get metadata for: {}", path.display());
                log::error!("{message}: {err:#}");
                match self.settings.exit {
                    true => return Err(err).context(message),
                    false => continue,
                }
            }

            log::trace!("got metadata for: {}", path.display());
            let meta = result.unwrap();

            let result = match meta.is_dir() {
                true => self.process_dir(&path),
                false => self.process_file(&path),
            };

            if let Err(err) = result {
                let message = format!("failed to process target: {}", path.display());
                log::error!("{message}: {err:#}");
                match self.settings.exit {
                    true => return Err(err).context(message),
                    false => continue,
                }
            }
        }

        log::trace!("processed targets");

        Ok(())
    }
}

impl Organiser {
    /// Process the provided directory
    fn process_dir(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing dir: {}", path.display());

        let directory = path
            .read_dir()
            .with_context(|| format!("failed to read directory: {}", path.display()))?;

        log::trace!("read dir: {}", path.display());

        for entry in directory {
            log::trace!("processing directory entry");
            if let Err(err) = entry {
                let message = format!("failed to read directory entry: {}", path.display());
                log::error!("{message}: {err:#}");
                match self.settings.exit {
                    true => return Err(err).context(message),
                    false => continue,
                }
            }

            let path = &entry.unwrap().path();

            let result = path.metadata();

            if let Err(err) = result {
                let message = format!("failed to get metadata for: {}", path.display());
                log::error!("{message}: {err:#}");
                match self.settings.exit {
                    true => return Err(err).context(message),
                    false => continue,
                }
            }

            log::trace!("got metadata for: {}", path.display());
            let meta = result.unwrap();

            let result = match [meta.is_dir(), self.settings.recursive] {
                [true, true] => self.process_dir(path),
                [true, false] => {
                    log::trace!("skipping subdirectory: {}", path.display());
                    continue;
                }
                [false, _] => self.process_file(path),
            };

            if let Err(err) = result {
                let message = format!("failed to process directory entry: {}", path.display());
                log::error!("{message}: {err:#}");
                match self.settings.exit {
                    true => return Err(err).context(message),
                    false => continue,
                }
            }
        }

        log::debug!("processed dir: {}", path.display());

        Ok(())
    }

    /// Process the provided file
    fn process_file(&self, path: &Path) -> anyhow::Result<()> {
        log::debug!("processing file: {}", path.display());

        let name = path.file_name().context("getting file name")?;
        let name = name.to_str().context("converting name to str")?;

        log::trace!("old name: {}", name);

        let comic: Meta = name.parse()?;
        let new_name = comic.to_string();

        log::trace!("new name: {}", &new_name);

        let output_dir = &self.settings.output;

        if output_dir.exists() && !output_dir.is_dir() {
            log::error!("output path is not a directory: {}", output_dir.display());
            anyhow::bail!("output path is not a directory");
        }

        let output_dir = match self.settings.series {
            true => output_dir.join(comic.series),
            false => output_dir.to_path_buf(),
        };

        log::trace!("output dir: {}", output_dir.display());

        match [output_dir.exists(), self.settings.dry_run] {
            [false, false] => {
                log::info!("creating output dir: {}", output_dir.display());
                fs::create_dir_all(&output_dir).context("creating output dir")?;
            }
            [false, true] => log::info!("would create output dir: {}", output_dir.display()),
            _ => log::trace!("output dir exists: {}", output_dir.display()),
        }

        let new_path = output_dir.join(new_name);

        log::trace!("new path: {}", new_path.display());

        if new_path.exists() {
            log::debug!("file already exists: {}", new_path.display());
            match [self.settings.dry_run, self.settings.force] {
                [true, true] => {
                    log::warn!("would overwrite existing file: {}", new_path.display());
                }
                [true, false] => {
                    log::error!("would skip existing file: {}", new_path.display());
                    anyhow::bail!("skipping existing file: {}", new_path.display());
                }
                [false, true] => {
                    log::warn!("overwriting existing file: {}", new_path.display());
                }
                [false, false] => {
                    log::error!("skipping existing file: {}", new_path.display());
                    anyhow::bail!("skipping existing file: {}", new_path.display());
                }
            }
        }

        match [self.settings.dry_run, self.settings.move_files] {
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

#[cfg(test)]
pub(crate) mod tests {
    use temp_dir::TempDir;
    use test_log::test;

    use super::*;

    #[test]
    fn test_process_multiple_targets() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");
        let source_sub_dir = source_dir.join("sub");

        let name_1 = "Test 001.cbz";
        let source_file_1 = source_dir.join(name_1);
        let output_file_1 = output_dir.join(name_1);

        let name_2 = "Test 002.cbz";
        let source_file_2 = source_sub_dir.join(name_2);
        let output_file_2 = output_dir.join(name_2);

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        std::fs::create_dir_all(&source_sub_dir).expect("should create source sub dir");
        fs::write(&source_file_1, "").expect("should create first source file");
        fs::write(&source_file_2, "").expect("should create second source file");

        let processor = Organiser::new(Settings {
            output: output_dir,
            ..Default::default()
        });

        let targets = vec![source_file_1.clone(), source_sub_dir.clone()];

        processor.process(targets).expect("should process");

        assert!(
            source_file_1.exists(),
            "source file should still exist: {}",
            source_file_1.display()
        );
        assert!(
            output_file_1.exists(),
            "output file should have been created: {}",
            output_file_1.display()
        );
        assert!(
            source_file_2.exists(),
            "source file should still exist: {}",
            source_file_2.display()
        );
        assert!(
            output_file_2.exists(),
            "output file should have been created: {}",
            output_file_2.display()
        );
    }

    #[test]
    fn test_process_recursive() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");
        let source_sub_dir = source_dir.join("sub");

        let name_1 = "Test 001.cbz";
        let source_file_1 = source_dir.join(name_1);
        let output_file_1 = output_dir.join(name_1);

        let name_2 = "Test 002.cbz";
        let source_file_2 = source_sub_dir.join(name_2);
        let output_file_2 = output_dir.join(name_2);

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        std::fs::create_dir_all(&source_sub_dir).expect("should create source sub dir");
        fs::write(&source_file_1, "").expect("should create first source file");
        fs::write(&source_file_2, "").expect("should create second source file");

        let processor = Organiser::new(Settings {
            output: output_dir,
            recursive: true,
            ..Default::default()
        });

        let targets = vec![source_dir];

        processor.process(targets).expect("should process");

        assert!(
            source_file_1.exists(),
            "source file should still exist: {}",
            source_file_1.display()
        );
        assert!(
            output_file_1.exists(),
            "output file should have been created: {}",
            output_file_1.display()
        );
        assert!(
            source_file_2.exists(),
            "source file should still exist: {}",
            source_file_2.display()
        );
        assert!(
            output_file_2.exists(),
            "output file should have been created: {}",
            output_file_2.display()
        );
    }

    #[test]
    fn test_process_non_recursive() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");
        let source_sub_dir = source_dir.join("sub");

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        std::fs::create_dir_all(&source_sub_dir).expect("should create source sub dir");

        let name = "Test 001.cbz";
        let source_file = source_sub_dir.join(name);
        let output_file = output_dir.join(name);

        fs::write(&source_file, "").expect("should create second source file");

        let processor = Organiser::new(Settings {
            output: output_dir,
            ..Default::default()
        });

        let targets = vec![source_dir];

        processor.process(targets).expect("should process");

        assert!(
            source_file.exists(),
            "source file should still exist: {}",
            source_file.display()
        );
        assert!(
            !output_file.exists(),
            "output file should not have been created: {}",
            output_file.display()
        );
    }

    #[test]
    fn test_process_move() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");

        let name = "Test 001.cbz";
        let source_file = source_dir.join(name);
        let output_file = output_dir.join(name);

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        fs::write(&source_file, "").expect("should create first source file");

        let processor = Organiser::new(Settings {
            output: output_dir,
            move_files: true,
            ..Default::default()
        });

        let targets = vec![source_dir];

        processor.process(targets).expect("should process");

        assert!(
            !source_file.exists(),
            "source file should not still exist: {}",
            source_file.display()
        );
        assert!(
            output_file.exists(),
            "output file should have been created: {}",
            output_file.display()
        );
    }

    #[test]
    fn test_process_exit() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");

        let name_1 = "Test 001.cbz";
        let source_file_1 = source_dir.join(name_1);
        let output_file_1 = output_dir.join(name_1);

        let name_2 = "Test 002.cbz";
        let source_file_2 = source_dir.join(name_2);
        let output_file_2 = output_dir.join(name_2);

        let contents = "contents";

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        fs::write(&source_file_1, contents).expect("should create first source file");
        fs::write(&output_file_1, "").expect("should create first output file");
        fs::write(&source_file_2, "").expect("should create second source file");

        let processor = Organiser::new(Settings {
            output: output_dir,
            move_files: true,
            exit: true,
            ..Default::default()
        });

        let targets = vec![source_file_1.clone(), source_file_2.clone()];

        processor.process(targets).expect_err("should exit early");

        assert!(
            source_file_1.exists(),
            "source file should still exist: {}",
            source_file_1.display()
        );
        assert!(
            output_file_1.exists(),
            "output file should exist: {}",
            output_file_1.display()
        );
        assert_eq!(
            fs::read_to_string(&output_file_1).expect("should read output file"),
            "",
            "output file should not have been overwritten"
        );
        assert!(
            source_file_2.exists(),
            "source file should still exist: {}",
            source_file_2.display()
        );
        assert!(
            !output_file_2.exists(),
            "output file should not have been created: {}",
            output_file_2.display()
        );
    }

    #[test]
    fn test_process_force() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");

        let name = "Test 001.cbz";
        let source_file = source_dir.join(name);
        let output_file = output_dir.join(name);

        let contents = "contents";

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        fs::write(&source_file, contents).expect("should create first source file");
        fs::write(&output_file, "").expect("should create first source file");

        let processor = Organiser::new(Settings {
            output: output_dir,
            move_files: true,
            force: true,
            ..Default::default()
        });

        let targets = vec![source_dir];

        processor.process(targets).expect("should process");

        assert!(
            !source_file.exists(),
            "source file should not still exist: {}",
            source_file.display()
        );
        assert!(
            output_file.exists(),
            "output file should have been created: {}",
            output_file.display()
        );
        assert_eq!(
            fs::read_to_string(&output_file).expect("should read output file"),
            contents,
            "output file should have been overwritten"
        );
    }

    #[test]
    fn test_process_no_force() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");

        let name = "Test 001.cbz";
        let source_file = source_dir.join(name);
        let output_file = output_dir.join(name);

        let contents = "contents";

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        fs::write(&source_file, contents).expect("should create first source file");
        fs::write(&output_file, "").expect("should create first source file");

        let processor = Organiser::new(Settings {
            output: output_dir,
            move_files: true,
            force: false,
            ..Default::default()
        });

        let targets = vec![source_dir];

        processor.process(targets).expect("should process");

        assert!(
            source_file.exists(),
            "source file should still exist: {}",
            source_file.display()
        );
        assert!(
            output_file.exists(),
            "output file should exist: {}",
            output_file.display()
        );
        assert_eq!(
            fs::read_to_string(&output_file).expect("should read output file"),
            "",
            "output file should not have been overwritten"
        );
    }

    #[test]
    fn test_process_series() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");

        let series = "Test";
        let series_dir = output_dir.join(series);

        let name = "Test 001.cbz";
        let source_file = source_dir.join(name);
        let output_file = series_dir.join(name);

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        std::fs::create_dir_all(&output_dir).expect("should create output dir");
        fs::write(&source_file, "").expect("should create first source file");

        assert!(
            !series_dir.exists(),
            "series dir should not exist before process: {}",
            series_dir.display()
        );

        let processor = Organiser::new(Settings {
            output: output_dir,
            series: true,
            ..Default::default()
        });

        let targets = vec![source_dir];

        processor.process(targets).expect("should process");

        assert!(
            source_file.exists(),
            "source file should still exist: {}",
            source_file.display()
        );
        assert!(
            series_dir.exists(),
            "series dir should have been created: {}",
            series_dir.display()
        );
        assert!(
            output_file.exists(),
            "output file should have been created: {}",
            output_file.display()
        );
    }

    #[test]
    fn test_process_creates_output_dir() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");

        let name = "Test 001.cbz";
        let source_file = source_dir.join(name);
        let output_file = output_dir.join(name);

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        fs::write(&source_file, "").expect("should create source file");

        assert!(
            !output_dir.exists(),
            "output dir should not exist before process: {}",
            output_dir.display()
        );

        let processor = Organiser::new(Settings {
            output: output_dir.clone(),
            ..Default::default()
        });

        let targets = vec![source_dir.clone()];

        processor.process(targets).expect("should process");

        assert!(
            output_dir.exists(),
            "output dir should have been created: {}",
            output_dir.display()
        );
        assert!(
            source_file.exists(),
            "source file should still exist: {}",
            source_file.display()
        );
        assert!(
            output_file.exists(),
            "output file should have been created: {}",
            output_file.display()
        );
    }

    #[test]
    fn test_process_dry_run() {
        let dir = TempDir::new().expect("should create temp dir");
        let source_dir = dir.child("source");
        let output_dir = dir.child("output");

        let series = "Test";
        let series_dir = output_dir.join(series);

        let name = "Test 001.cbz";
        let source_file = source_dir.join(name);
        let output_file = output_dir.join(name);

        std::fs::create_dir_all(&source_dir).expect("should create source dir");
        fs::write(&source_file, "").expect("should create source file");

        assert!(
            !output_dir.exists(),
            "output dir should not exist before process: {}",
            output_dir.display()
        );
        assert!(
            !series_dir.exists(),
            "series dir should not exist before process: {}",
            series_dir.display()
        );

        let processor = Organiser::new(Settings {
            output: output_dir.clone(),
            series: true,
            move_files: true,
            dry_run: true,
            force: true,
            ..Default::default()
        });

        let targets = vec![source_dir.clone()];

        processor.process(targets).expect("should process");

        assert!(
            !output_dir.exists(),
            "output dir should not be created: {}",
            output_dir.display()
        );
        assert!(
            !series_dir.exists(),
            "series dir should not be created: {}",
            series_dir.display()
        );
        assert!(
            source_file.exists(),
            "source file should still exist: {}",
            source_file.display()
        );
        assert!(
            !output_file.exists(),
            "output file should not be created: {}",
            output_file.display()
        );
    }
}
