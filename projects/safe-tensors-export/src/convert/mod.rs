use std::fs::remove_file;
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use url::Url;
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
                if let Err(e) = convert_file(path) {
                    log::error!("{}: {}", path.display(), e);
                }
            }
        }
        Ok(())
    }
    fn get_directory(&self) -> anyhow::Result<PathBuf> {
        let root = PathBuf::from(".");
        Ok(root)
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


fn convert_file(path: &Path, overwrite: bool) -> anyhow::Result<()> {
    if is_extension(path, "safetensors") {
        return Ok(());
    }
    if is_extension(path, "onnx") {
        let output = path.with_extension("safetensors");
        if output.exists() {
            if overwrite {
                remove_file(&output)?;
            } else {
                log::info!("Skip existing file: {}", Url::from_file_path(output).unwrap());
                return Ok(());
            }
        }
        let url1 = Url::from_file_path(path).unwrap();
        let url2 = Url::from_file_path(&output).unwrap();
        log::info!("Convert onnx: \n    {} \n => {}", url1, url2);
        convert_onnx(path, &output)?;
    }
    Ok(())
}

fn is_extension(path: &Path, extension: &str) -> bool {
    path.extension().map(|e| e.eq_ignore_ascii_case(extension)).unwrap_or(false)
}