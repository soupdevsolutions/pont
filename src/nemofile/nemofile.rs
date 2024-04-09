use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NemoFile {
    pub name: String,
    pub commands: Vec<String>,
    pub ignore: Vec<String>,
}

impl NemoFile {
    pub fn parse(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?; 
        let data: NemoFile = serde_yaml::from_str(&content)?;
        Ok(data)    
    }
}
