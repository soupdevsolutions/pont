use std::path::PathBuf;

use super::NemoFile;

pub struct NemoProject {
    pub name: String,
    pub nemofile: NemoFile,
    pub path: PathBuf,
}

impl NemoProject {
    pub fn create(name: &str, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir(&path)?;
        let nemofile = NemoFile::create_default(&name, &path)?;
        Ok(Self {
            name: name.to_string(),
            nemofile,
            path,
        })
    }
}
