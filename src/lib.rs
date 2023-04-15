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
    renamed: bool,
}

impl SubtitleFile {
    /// Creates a new instance of SubtitleFile making sure that the provided path
    /// has the subtitle file name extension
    ///
    /// # Error
    /// This method returns an error when th provided path does not have a valid
    /// subtitle file extension.
    pub fn new(subtitle_file_path: path::PathBuf) -> Result<Self, SubtitleFileError> {
        if let Some(extension) = subtitle_file_path.extension() {
            if extension == "srt" {
                return Ok(Self {
                    subtitle_file_path,
                    renamed: false,
                });
            }
        }
        Err(SubtitleFileError::InvalidSubtileFileName)
    }

    /// Renames the subtitle file using the name of a movie file
    ///
    /// # Errors
    /// This function return errors when the rename operation fails due to permission, etc, or
    /// when the subtitle file name and the movie file name have no matching season and episode
    /// signatures, that is the word S01EO5 that imply that the files are of the First season
    /// at episode Five
    pub fn rename_using_movie_file(
        &mut self,
        movie_file: &MovieFile,
    ) -> Result<(), SubtitleFileError> {
        let movie_name = movie_file.get_path();
        let subtitle_file_name = &self.subtitle_file_path;

        if let MatchSignature::Match =
            episode_name_signature_check(movie_name.as_os_str(), subtitle_file_name.as_os_str())
        {
            let mut new_subtitle_file_name = path::PathBuf::from(movie_name);
            new_subtitle_file_name.set_extension("srt");

            if let Err(err) = fs::rename(&self.subtitle_file_path, new_subtitle_file_name) {
                return Err(SubtitleFileError::FileSystem(err.to_string()));
            }
            self.renamed = true;
            return Ok(());
        }
        Err(SubtitleFileError::MovieSubFileNamesMismatch)
    }

    /// Returns true if the subtitle file is already renamed and vice versa
    pub fn is_renamed(&self) -> bool {
        self.renamed
    }
}

impl std::fmt::Display for SubtitleFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_to_display = self.subtitle_file_path.to_string_lossy();
        write!(f, "{}", path_to_display)
    }
}

/// Error that can be returned when performing operations related to a movie file
#[derive(Debug, Error)]
pub enum MovieFileError {
    /// This error is returned when trying to create a movie instance while the provided
    /// path does not have a movie extension, that is .mp4 or .mkv
    #[error("The movie name does not end with 'mkv' or 'mp4' extensions")]
    InvalidMovieFileName,
}

/// Struct representing a movie file
#[derive(Debug)]
pub struct MovieFile {
    movie_file_path: path::PathBuf,
}

impl MovieFile {
    /// Creates a new instance of MovieFile
    ///
    /// # Errors
    /// This method will return an error when the path passed does not have a valid movie
    /// extension ie .mp4, mkv
    pub fn new(movie_file_path: path::PathBuf) -> Result<Self, MovieFileError> {
        if let Some(extension) = movie_file_path.extension() {
            if extension == "mkv" || extension == "mp4" {
                return Ok(Self { movie_file_path });
            }
        }
        Err(MovieFileError::InvalidMovieFileName)
    }

    /// Returns the path of the MovieFile
    fn get_path(&self) -> &path::Path {
        &self.movie_file_path
    }
}

impl std::fmt::Display for MovieFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_to_display = self.movie_file_path.to_string_lossy();
        write!(f, "{}", path_to_display)
    }
}
