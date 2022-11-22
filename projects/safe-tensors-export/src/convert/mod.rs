use std::fs::{remove_file};
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::WalkDir;
use safe_tensors_loader::convert_onnx;

use crate::Application;

impl Application {
    pub fn run(&self) -> anyhow::Result<()> {
        let glob = self.get_glob()?;
        for entry in WalkDir::new(self.get_directory()?) {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && glob.is_match(path) {
                log::info!("converting: {}", path.display());
                if let Err(e) = convert_file(path) {
                    log::error!("{}: {}", path.display(), e);
                }
            }
        }
        Ok(())
    }
    fn get_directory(&self) -> anyhow::Result<PathBuf> {
        let root = PathBuf::from(".");
        let directory = match &self.directory {
            Some(d) if d.is_absolute() => d.clone(),
            Some(d) if d.starts_with("/") => d.clone(),
            Some(d) => root.join(d),
            None => root,
        };
        let directory = directory.canonicalize()?;
        if !directory.is_dir() {
            Err(anyhow::Error::msg("path must a directory"))?
        }
        Ok(directory)
    }
    fn get_glob(&self) -> anyhow::Result<GlobSet> {
        let mut glob = GlobSetBuilder::new();
        let pattern = match &self.pattern {
            Some(p) => p,
            None => "*.onnx",
        };
        for i in pattern.split(',') {
            glob.add(Glob::new(i.trim())?);
        }
        Ok(glob.build()?)
    }
}


fn convert_file(path: &Path) -> anyhow::Result<()> {
    if  path.ends_with("safetensors") {
        return Ok(());
    }
    if path.ends_with("onnx") {
        let output = path.with_extension("safetensors");
        if output.exists() {
            remove_file(&output)?;
        }
        convert_onnx(path, &output)?;
    }
    Ok(())
}
