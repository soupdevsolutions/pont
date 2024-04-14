use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    pub path: PathBuf,
}

impl Directory {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.is_dir() {
            return Err("Path is not a directory".into());
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

    pub fn copy_content(&self, target: &Path) -> Result<Self, Box<dyn std::error::Error>> {
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
