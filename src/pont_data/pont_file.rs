use std::path::Path;

use serde::{Deserialize, Serialize};

use super::PontFileError;

#[derive(Debug, Serialize, Deserialize)]
pub struct PontFile {
    pub name: String,
    pub commands: Vec<String>,
    pub ignore: Vec<String>,
}

impl PontFile {
    pub fn empty(name: &str) -> Self {
        Self {
            name: name.to_string(),
            commands: vec![],
            ignore: vec![],
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
