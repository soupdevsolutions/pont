use std::path::PathBuf;

use super::NemoFile;

#[derive(Debug)]
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

    pub fn load(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path.join("nemofile.yaml"))?;
        let nemofile: NemoFile = serde_yaml::from_reader(file)?;
        Ok(Self {
            name: nemofile.name.clone(),
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
