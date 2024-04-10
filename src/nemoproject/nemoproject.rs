use std::{fs, path::{Path, PathBuf}};

use walkdir::WalkDir;

use super::{NemoFile, Source};

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

    pub fn load(source: Source, target: &Path) -> Result<Self, Box<dyn std::error::Error>> {
         match source {
            Source::GitRepository(url) => {
                let auth = auth_git2::GitAuthenticator::default();
                let _repo = auth.clone_repo(&url, target)?;
            }
            Source::LocalDirectory(path) => {
                for entry in WalkDir::new(&path) {
                    let entry = entry?;
                    let from = entry.path();
                    let to = target.join(from.strip_prefix(&path)?);

                    if entry.file_type().is_dir() {
                        if let Err(e) = fs::create_dir(to) {
                            eprintln!("Error creating directory: {}", e);
                        }
                    } else if entry.file_type().is_file() {
                        fs::copy(from, to)?;
                    }
                }
            }
        };
        let nemofile = NemoFile::parse(&target.join("nemofile.yaml"))?;
        let name = nemofile.name.clone();
        Ok(Self {
            name,
            nemofile,
            path: target.into(),
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
