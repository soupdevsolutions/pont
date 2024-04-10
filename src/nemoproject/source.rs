use std::path::PathBuf;

use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use claim::assert_ok;

    #[test]
    fn test_git_repository_parse() {
        let url = Url::parse("https://github.com/soupdevsolutions/nemo").unwrap();

        let source = Source::parse(&url);

        assert_ok!(&source);
        assert_eq!(Source::GitRepository(url), source.unwrap());
    }

    #[test]
    fn test_local_directory_parse() {
        let path = "/path/to/project";
        let url = Url::parse(&format!("file://{}", path)).unwrap();

        let source = Source::parse(&url);

        assert_ok!(&source);
        assert_eq!(Source::LocalDirectory(PathBuf::from(path)), source.unwrap());
    }

    #[test]
    fn test_unsupported_source_type() {
        let url = Url::parse("ftp://example.com").unwrap();

        let source = Source::parse(&url);

        assert!(source.is_err());
    }
}
