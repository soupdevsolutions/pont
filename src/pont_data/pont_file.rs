use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::env::{get_env_variable, CARGO_PACKAGE_VERSION};

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
}
