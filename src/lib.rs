#![deny(missing_docs)]

//! Crate containing the required structs to create movie files and subtitle files
//!
//! As the main goal of this library is to provide an easy way to rename subtitle files
//! with respect to their movie files, both subtitle files and movies files keep tab on
//! when they are used, that is when a movie file has been matched and when a subtitle
//! file has been renamed. This helps prevent unecessary reuse of these struct.

use anyhow::Result;
use name_signature::{episode_name_signature_check, MatchSignature};
use std::fs;
use std::path;
use thiserror::Error;

mod name_signature;

const SUBTITLE_FILE_EXTENSION: &str = "srt";
const MOVIE_FILE_EXTENSIONS: &[&str] = &["mp4", "mkv", "flv", "avi", "3gp", "mov"];

/// Error that can be returned when performing operations related to a subtitle file
#[derive(Debug, Error)]
pub enum SubtitleFileError {
    /// This error is returned when a subtitle file name does not end with the typical
    /// subtitle file extension ".srt"
    #[error("The subtitle file name does not end with extension 'srt'")]
    InvalidSubtileFileName,

    /// This error is returned when the subtitle file name and the movie file name do not match
    /// in terms of their signature when trying to rename the subtitle file
    #[error("The movie file name and subtitle file name don't match in terms of their signatures")]
    MovieSubFileNamesMismatch,

    /// This error is returned when a error is return by fs::rename() function
    #[error("There is an error related to the filesystem: (0)")]
    FileSystem(String),
}

/// Struct representing a subtitle file
#[derive(Debug)]
pub struct SubtitleFile {
    subtitle_file_path: path::PathBuf,
}

impl SubtitleFile {
    /// Renames the subtitle file using the name of a movie file
    ///
    /// # Errors
    /// This function return errors when the rename operation fails due to permission, etc, or
    /// when the subtitle file name and the movie file name have no matching season and episode
    /// signatures, that is the word S01EO5 that imply that the files are of the First season
    /// at episode Five
    pub fn rename_using_movie_file(&self, movie_file: &MovieFile) -> Result<(), SubtitleFileError> {
        if let MatchSignature::Match = episode_name_signature_check(
            movie_file.get_path().as_os_str(),
            self.subtitle_file_path.as_os_str(),
        ) {
            let mut new_subtitle_file_name = path::PathBuf::from(movie_file.get_path());
            new_subtitle_file_name.set_extension(SUBTITLE_FILE_EXTENSION);

            if let Err(err) = fs::rename(&self.subtitle_file_path, new_subtitle_file_name) {
                return Err(SubtitleFileError::FileSystem(err.to_string()));
            }
            return Ok(());
        }
        Err(SubtitleFileError::MovieSubFileNamesMismatch)
    }
}

impl TryFrom<path::PathBuf> for SubtitleFile {
    type Error = SubtitleFileError;

    fn try_from(value: path::PathBuf) -> std::result::Result<Self, Self::Error> {
        if let Some(extension) = value.extension() {
            if extension == SUBTITLE_FILE_EXTENSION {
                return Ok(Self {
                    subtitle_file_path: value,
                });
            }
        }
        Err(SubtitleFileError::InvalidSubtileFileName)
    }
}

impl std::fmt::Display for SubtitleFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_to_display = self.subtitle_file_path.to_string_lossy();
        write!(f, "{}", path_to_display)
    }
}

/// Struct representing a movie file
#[derive(Debug)]
pub struct MovieFile(path::PathBuf);

impl MovieFile {
    /// # Constructs a MovieFile
    ///
    /// This method takes an optional vec of extensions to include when constructing
    /// the MoviesFile, otherwise when the argument is None it will default to the
    /// built in extension.
    /// Returns None when the path provided is of unknown extension
    pub fn new(value: path::PathBuf, extra_extensions: Option<&Vec<String>>) -> Option<Self> {
        if let Some(extension) = value.extension() {
            // Checking the extra extensions first
            if let Some(extra_extensions) = extra_extensions {
                if extra_extensions
                    .iter()
                    .any(|val| *val == extension.to_string_lossy())
                {
                    return Some(Self(value));
                }
            }
            // Checking the default extensions when no extra extensions are provided
            if MOVIE_FILE_EXTENSIONS.iter().any(|val| *val == extension) {
                return Some(Self(value));
            }
        }
        None
    }

    /// Returns the path of the MovieFile
    fn get_path(&self) -> &path::Path {
        &self.0
    }
}

impl std::fmt::Display for MovieFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_to_display = self.0.to_string_lossy();
        write!(f, "{}", path_to_display)
    }
}

#[cfg(test)]
mod tests {
    use super::MovieFile;
    use crate::MOVIE_FILE_EXTENSIONS;
    use std::path;

    #[test]
    fn movie_file_creation_with_default_extension_test() {
        let movie_paths: Vec<path::PathBuf> = MOVIE_FILE_EXTENSIONS
            .iter()
            .map(|ext| path::PathBuf::from(format!("mov.{}", ext)))
            .collect();

        let total_movie_files_created = movie_paths
            .iter()
            .take_while(|path| MovieFile::new(path.into(), None).is_some())
            .count();

        assert_eq!(total_movie_files_created, movie_paths.len())
    }

    #[test]
    fn movie_file_creation_with_extra_extension_test() {
        let extra_extension: Vec<String> = ('a'..'z').map(|ext| ext.to_string()).collect();

        let movie_paths: Vec<path::PathBuf> = extra_extension
            .iter()
            .map(|ext| path::PathBuf::from(format!("mov.{}", ext)))
            .collect();

        let total_movie_files_created = movie_paths
            .iter()
            .take_while(|path| MovieFile::new(path.into(), Some(&extra_extension)).is_some())
            .count();

        assert_eq!(total_movie_files_created, movie_paths.len())
    }
}
