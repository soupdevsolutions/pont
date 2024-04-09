use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NemoFile {
    pub name: String,
    pub commands: Vec<String>,
    pub ignore: Vec<String>,
}

impl NemoFile {
    pub fn empty(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            name: name.to_string(),
            commands: vec![],
            ignore: vec![],
        })
    }

    pub fn parse(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let data: NemoFile = serde_yaml::from_str(&content)?;
        Ok(data)
    }
}
