use std::path::{Path, PathBuf};

use globset::{Glob, GlobMatcher};
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

    fn compile_glob_matchers(&self) -> Result<Vec<GlobMatcher>, PontFileError> {
        if self.ignore.is_none() {
            return Ok(vec![]);
        }
        let mut matchers = vec![];
        for pattern in self.ignore.clone().unwrap() {
            let matcher = Glob::new(&pattern)?.compile_matcher();
            matchers.push(matcher);
        }
        Ok(matchers)
    }

    pub fn compile_ignored_files(&self, files: &[PathBuf]) -> Result<Vec<PathBuf>, PontFileError> {
        if self.ignore.is_none() {
            return Ok(vec![]);
        } 
        let mut ignored_files = vec![];
        let matchers = self.compile_glob_matchers()?;
        for file in files {
            let mut ignore = false;
            for matcher in &matchers {
                if matcher.is_match(&file) {
                    ignore = true;
                    break;
                }
            }
            if !ignore {
                ignored_files.push(file.into());
            }
        }
        Ok(ignored_files)
    }
}
