use std::path::PathBuf;

use super::NemoFile;

pub struct NemoProject {
    pub name: String,
    pub nemofile: NemoFile,
    pub path: PathBuf,
}

impl NemoProject {
    pub fn new(name: &str, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let nemofile = NemoFile::empty(&name)?;
        Ok(Self {
            name: name.to_string(),
            nemofile,
            path,
        })
    }

    pub fn save(&self, new_directory: bool) -> Result<(), Box<dyn std::error::Error>> {
        if new_directory {
            std::fs::create_dir(&self.path)?;
        }
        let file = std::fs::File::create(self.path.join("nemofile.yaml"))?;
        serde_yaml::to_writer(file, &self.nemofile)?;
        Ok(())
    }
}
