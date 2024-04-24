use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirectoryError {
    #[error("The specified path is not a directory: {0}")]
    NotADirectory(PathBuf),
    #[error("Could not get current directory.")]
    CurrentDir,
    #[error("Could not read directory: {0}")]
    ReadDir(PathBuf),
    #[error("An IO error occured : {0}.")]
    IoError(#[from] std::io::Error),
    #[error("An error occured while processing the files: {0}")]
    ProcessingError(#[from] std::path::StripPrefixError),
}
