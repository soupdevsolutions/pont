use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    pub path: PathBuf,
}

impl Directory {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.is_dir() {
            return Err(format!("Path {:?} is not a directory", path).into());
        }

        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn current() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            path: std::env::current_dir()?,
        })
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn create_subdir(&self, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = self.path.join(name);
        std::fs::create_dir(&path)?;

        Ok(Self { path })
    }

    pub fn copy_files(&self, target: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        for entry in walkdir::WalkDir::new(&self.path) {
            let entry = entry?;
            let from = entry.path();
            let to = target.join(from.strip_prefix(&self.path)?);

            if entry.file_type().is_dir() {
                if let Err(e) = std::fs::create_dir(to) {
                    eprintln!("Error creating directory: {}", e);
                }
            } else if entry.file_type().is_file() {
                std::fs::copy(from, to)?;
            }
        }

        Ok(Self {
            path: target.into(),
        })
    }

    pub fn get_files(
        &self,
        ignored_dirs: Option<&[String]>,
    ) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut files = vec![];
        let mut director_it = walkdir::WalkDir::new(&self.path).into_iter();

        loop {
            let entry = match director_it.next() {
                Some(Ok(entry)) => entry,
                Some(Err(e)) => return Err(e.into()),
                None => break,
            };

            let file_name = entry.file_name().to_string_lossy().to_string();
            let should_skip = ignored_dirs.is_some() && ignored_dirs.unwrap().contains(&file_name);

            if entry.file_type().is_dir() && should_skip {
                director_it.skip_current_dir();
                continue;
            }

            if entry.file_type().is_file() {
                files.push(entry.path().into());
            }
        }
        Ok(files)
    }
}

impl TryFrom<PathBuf> for Directory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::new(&path)
    }
}

impl TryFrom<&Path> for Directory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Self::new(path)
    }
}

impl TryFrom<&str> for Directory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::new(Path::new(path))
    }
}
