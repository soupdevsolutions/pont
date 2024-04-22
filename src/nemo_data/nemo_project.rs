use crate::file_management::Directory;
use std::{
    fs::{remove_file, File},
    io::{Read, Write},
};

use super::{NemoFile, Source, NEMO_FILE_NAME};

#[derive(Debug)]
pub struct NemoProject {
    pub name: String,
    pub nemofile: NemoFile,
    pub directory: Directory,
}

impl NemoProject {
    pub fn new(name: &str, directory: &Directory) -> Result<Self, Box<dyn std::error::Error>> {
        let nemofile = NemoFile::empty(name)?;
        Ok(Self {
            name: name.to_string(),
            nemofile,
            directory: directory.clone(),
        })
    }

    pub fn load(source: Source, target: &Directory) -> Result<Self, Box<dyn std::error::Error>> {
        match source {
            Source::GitRepository(url) => {
                let auth = auth_git2::GitAuthenticator::default();
                let _repo = auth.clone_repo(url, &target.path)?;
            }
            Source::LocalDirectory(path) => {
                path.copy_files(&target.path())?;
            }
        };

        let nemofile = NemoFile::parse(&target.path.join(NEMO_FILE_NAME))?;
        Ok(Self {
            name: target.name(),
            nemofile,
            directory: target.clone(),
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(self.directory.path.join(NEMO_FILE_NAME))?;
        serde_yaml::to_writer(file, &self.nemofile)?;
        Ok(())
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ignore_files = self.nemofile.ignore.clone();

        let files = self.directory.get_files(Some(&ignore_files))?;
        for f in &files {
            let mut file = File::open(f.clone())?;
            let file_name = f.file_name().unwrap().to_string_lossy().to_string();

            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let content = content.replace(&self.nemofile.name, &self.name);

            let mut file = File::create(f)?;
            file.write_all(content.as_bytes())?;

            if file_name.contains(&self.nemofile.name) {
                let new_name = file_name.replace(&self.nemofile.name, &self.name);
                let new_path = f.parent().unwrap().join(new_name);
                std::fs::rename(f, new_path)?;
            }
        }

        self.nemofile.commands.iter().for_each(|command| {
            let mut cmd = std::process::Command::new("sh");
            cmd.arg("-c").arg(command);
            let _status = cmd.status().expect("Failed to execute command");
        });

        remove_file(self.directory.path.join(NEMO_FILE_NAME))?;

        Ok(())
    }
}

impl TryFrom<&Directory> for NemoProject {
    type Error = Box<dyn std::error::Error>;

    fn try_from(directory: &Directory) -> Result<Self, Self::Error> {
        let name = directory.name();
        let nemofile = NemoFile::empty(&name)?;
        Ok(Self {
            name: name.to_string(),
            nemofile,
            directory: directory.clone(),
        })
    }
}
