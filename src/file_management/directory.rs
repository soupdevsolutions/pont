use std::path::{Path, PathBuf};

use super::DirectoryError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    pub path: PathBuf,
}

impl Directory {
    pub fn new(path: &Path) -> Result<Self, DirectoryError> {
        if !path.is_dir() {
            return Err(DirectoryError::NotADirectory(path.into()));
        }

        Ok(Self { path: path.into() })
    }

    pub fn current() -> Result<Self, DirectoryError> {
        Ok(Self {
            path: std::env::current_dir().map_err(|_| DirectoryError::CurrentDir)?,
        })
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn create_subdir(&self, name: &str) -> Result<Self, DirectoryError> {
        let path = self.path.join(name);
        std::fs::create_dir(&path)?;

        Ok(Self { path })
    }

    pub fn copy_files(&self, target: &Path) -> Result<Self, DirectoryError> {
        for entry in walkdir::WalkDir::new(&self.path) {
            let entry = entry.map_err(|_| DirectoryError::ReadDir(self.path.clone()))?;
            let from = entry.path();
            let to = target.join(from.strip_prefix(&self.path)?);

            if entry.file_type().is_dir() {
                if let Err(_e) = std::fs::create_dir(to.clone()) {
                    continue;
                }
            } else if entry.file_type().is_file() {
                std::fs::copy(from, to.clone())?;
            }
        }
        Ok(Self {
            path: target.into(),
        })
    }

    pub fn get_files(&self) -> Result<Vec<PathBuf>, DirectoryError> {
        let path = self.path.to_string_lossy().to_string();
        let mut files = vec![];
        let mut director_it = walkdir::WalkDir::new(&self.path).into_iter();

        loop {
            let entry = match director_it.next() {
                Some(Ok(entry)) => entry,
                Some(Err(e)) => {
                    eprintln!("Error: {}", e);
                    return Err(DirectoryError::ReadDir(self.path.clone()));
                }
                None => break,
            };
            files.push(entry.path().strip_prefix(&path).unwrap().into());
        }
        Ok(files)
    }
}

impl TryFrom<PathBuf> for Directory {
    type Error = DirectoryError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::new(&path)
    }
}

impl TryFrom<&Path> for Directory {
    type Error = DirectoryError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Self::new(path)
    }
}

impl TryFrom<&str> for Directory {
    type Error = DirectoryError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::new(Path::new(path))
    }
}
