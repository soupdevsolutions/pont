use url::Url;

use crate::file_management::Directory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    GitRepository(Url),
    LocalDirectory(Directory),
}

impl Source {
    pub fn parse(source: &Url) -> Result<Self, Box<dyn std::error::Error>> {
        match source.scheme() {
            "http" | "https" => Ok(Self::GitRepository(source.clone())),
            "file" => Ok(Self::LocalDirectory(Directory::new(
                &source
                    .to_file_path()
                    .map_err(|e| format!("Invalid file path: {:?}", e))?,
            )?)),
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
        let path = "/";
        let url = Url::parse(&format!("file://{}", path)).unwrap();

        let source = Source::parse(&url);

        assert_ok!(&source);
        assert_eq!(
            Source::LocalDirectory(Directory::try_from(path).unwrap()),
            source.unwrap()
        );
    }

    #[test]
    fn test_unsupported_source_type() {
        let url = Url::parse("ftp://example.com").unwrap();

        let source = Source::parse(&url);

        assert!(source.is_err());
    }
}
