use std::path::PathBuf;

use url::Url;

pub enum Source {
    GitRepository(Url),
    LocalDirectory(PathBuf),
}

impl Source {
    pub fn parse(source: &Url) -> Result<Self, Box<dyn std::error::Error>> {
        match source.scheme() {
            "http" | "https" => Ok(Self::GitRepository(source.clone())),
            "file" => Ok(Self::LocalDirectory(PathBuf::from(source.path()))),
            _ => Err("Unsupported source type".into()),
        }
    }
}
