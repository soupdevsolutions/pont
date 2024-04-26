use crate::file_management::Directory;
use std::{
    fs::{remove_file, File},
    io::{Read, Write},
};

use super::{PontFile, PontProjectError, Source, PONT_FILE_NAME};

#[derive(Debug)]
pub struct PontProject {
    pub name: String,
    pub pontfile: PontFile,
    pub directory: Directory,
}

impl PontProject {
    pub fn new(name: &str, directory: &Directory) -> Self {
        let pontfile = PontFile::empty(name);
        Self {
            name: name.to_string(),
            pontfile,
            directory: directory.clone(),
        }
    }

    pub fn load(source: Source, target: &Directory) -> Result<Self, PontProjectError> {
        match source {
            Source::GitRepository(url) => {
                let auth = auth_git2::GitAuthenticator::default();
                let _repo = auth.clone_repo(url, &target.path)?;
            }
            Source::LocalDirectory(path) => {
                path.copy_files(&target.path())?;
            }
        };

        let pontfile = PontFile::parse(&target.path.join(PONT_FILE_NAME))?;
        Ok(Self {
            name: target.name(),
            pontfile,
            directory: target.clone(),
        })
    }

    pub fn save(&self) -> Result<(), PontProjectError> {
        self.pontfile
            .save(&self.directory.path.join(PONT_FILE_NAME))?;
        Ok(())
    }

    pub fn build(&self) -> Result<(), PontProjectError> {
        let ignore_files = self.pontfile.ignore.clone();

        let files = self.directory.get_files(ignore_files.as_deref())?;
        for f in &files {
            let mut file = File::open(f.clone())?;
            let file_name = f.file_name().unwrap().to_string_lossy().to_string();

            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let content = content.replace(&self.pontfile.name, &self.name);

            let mut file = File::create(f)?;
            file.write_all(content.as_bytes())?;

            if file_name.contains(&self.pontfile.name) {
                let new_name = file_name.replace(&self.pontfile.name, &self.name);
                let new_path = f.parent().unwrap().join(new_name);
                std::fs::rename(f, new_path)?;
            }
        }

        if let Some(commands) = &self.pontfile.commands {
            commands.iter().for_each(|command| {
                let mut cmd = std::process::Command::new("sh");
                cmd.arg("-c").arg(command);
                let _status = cmd.status().expect("Failed to execute command");
            });
        }
        remove_file(self.directory.path.join(PONT_FILE_NAME))?;

        Ok(())
    }
}

impl From<&Directory> for PontProject {
    fn from(directory: &Directory) -> Self {
        let name = directory.name();
        let pontfile = PontFile::empty(&name);
        Self {
            name: name.to_string(),
            pontfile,
            directory: directory.clone(),
        }
    }
}
