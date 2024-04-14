use crate::file_management::Directory;

use super::{NemoFile, Source};

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
                let dir = Directory::try_from(path)?;
                dir.copy_content(&target.path())?;
            }
        };

        let nemofile = NemoFile::parse(&target.path.join("nemofile.yaml"))?;
        let name = nemofile.name.clone();
        Ok(Self {
            name,
            nemofile,
            directory: target.clone(),
        })
    }

    pub fn save(&self, new_directory: bool) -> Result<(), Box<dyn std::error::Error>> {
        if new_directory {
            std::fs::create_dir(&self.directory.path)?;
        }
        let file = std::fs::File::create(self.directory.path.join("nemofile.yaml"))?;
        serde_yaml::to_writer(file, &self.nemofile)?;
        Ok(())
    }

    pub fn build(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // replace all instances of the template name with the new project name

        // run all the commands from the nemofile

        // delete the nemofile

        Ok(())
    }
}

impl TryFrom<&Directory> for NemoProject {
    type Error = Box<dyn std::error::Error>;

    fn try_from(directory: &Directory) -> Result<Self, Self::Error> {
        let nemofile = NemoFile::parse(&directory.path.join("nemofile.yaml"))?;
        let name = nemofile.name.clone();
        Ok(Self {
            name,
            nemofile,
            directory: directory.clone(),
        })
    }
}
