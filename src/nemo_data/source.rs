use url::Url;

use crate::file_management::Directory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    GitRepository(String),
    LocalDirectory(Directory),
}

impl Source {
    pub fn parse(source: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let source = source.into();
        let scheme = Url::parse(&source)?.scheme().to_string();
        match scheme.trim() {
            "http" | "https" => Ok(Self::GitRepository(source)),
            "file" => {
                let current_dir = Directory::current()?;
                let source = source.strip_prefix("file://").unwrap_or(&source);
                let source = current_dir.path.join(source);
                Ok(Self::LocalDirectory(Directory::new(&source)?))
            }
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

        let source = Source::parse(url.clone());

        assert_ok!(&source);
        assert_eq!(Source::GitRepository(url.to_string()), source.unwrap());
    }

    #[test]
    fn test_local_directory_parse() {
        let path = "/./";
        let url = Url::parse(&format!("file://{}", path)).unwrap();

        let source = Source::parse(url);

        assert_ok!(&source);
        assert_eq!(
            Source::LocalDirectory(Directory::try_from(path).unwrap()),
            source.unwrap()
        );
    }

    #[test]
    fn test_unsupported_source_type() {
        let url = Url::parse("ftp://example.com").unwrap();

        let source = Source::parse(url);

        assert!(source.is_err());
    }
}
