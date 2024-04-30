use thiserror::Error;

use crate::file_management::DirectoryError;

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Could not parse {0} to a valid URL.")]
    ParseError(String),
    #[error("Error while handling provided file source.")]
    FileSourceError(#[from] DirectoryError),
}

#[derive(Debug, Error)]
pub enum PontFileError {
    #[error("Could not read file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("Could not parse file: {0}")]
    ParseFile(#[from] serde_yaml::Error),
    #[error("Could not compile glob pattern: {0}")]
    CompileGlobPattern(#[from] globset::Error),
}

#[derive(Debug, Error)]
pub enum PontProjectError {
    #[error("Could not create project directory.")]
    CreateProjectDir(#[from] DirectoryError),
    #[error("An error occured while handling the pont file.")]
    PontFileError(#[from] PontFileError),
    #[error("Could not clone Git repository.")]
    CloneGitRepo(#[from] git2::Error),
    #[error("An error occured while working with files.")]
    FileError(#[from] std::io::Error),
}
