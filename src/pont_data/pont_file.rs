use std::path::{Path, PathBuf};

use globset::{Glob, GlobSetBuilder};
use serde::{Deserialize, Serialize};


use super::PontFileError;

#[derive(Debug, Serialize, Deserialize)]
pub struct PontFile {
    pub version: String,
    pub name: String,
    pub commands: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
}

impl PontFile {
    pub fn empty(name: &str) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();
        Self {
            name: name.to_string(),
            version,
            commands: Some(vec![]),
            ignore: Some(vec![]),
        }
    }

    pub fn parse(path: &Path) -> Result<Self, PontFileError> {
        let content = std::fs::read_to_string(path)?;
        let data: PontFile = serde_yaml::from_str(&content)?;
        Ok(data)
    }

    pub fn save(&self, path: &Path) -> Result<(), PontFileError> {
        let file = std::fs::File::create(path)?;
        serde_yaml::to_writer(file, self)?;
        Ok(())
    }

    pub fn compile_ignored_files(&self, files: &[PathBuf]) -> Result<Vec<PathBuf>, PontFileError> {
        if self.ignore.is_none() {
            return Ok(vec![]);
        } 
        let mut ignored_files = vec![];

        let mut builder = GlobSetBuilder::new();
        for pattern in self.ignore.clone().unwrap() {
            let pattern = Glob::new(&pattern)?;
            builder.add(pattern);
        }
        let glob_set = builder.build()?;

        for file in files {
            if file.to_string_lossy().is_empty() {
                continue;
            }
            if glob_set.is_match(file) {
                ignored_files.push(file.clone());
            }
        }
        Ok(ignored_files)
    }
}
