use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs;
use std::path;
use sub_auto_rename::*;

#[derive(Parser)]
struct Cli {
    /// The directory where there are all the episodes and
    /// and their corresponding subtile files
    episodes_subs_directory: path::PathBuf,

    /// Whether to ignore the difference in the number of files between subtitle files
    /// and episodes files as the default behaviour expects them to be of equal amount.
    #[clap(short, long)]
    ignore_number_difference: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut movie_files = Vec::new();
    let mut subtitle_files = Vec::new();

    for dir_entry in fs::read_dir(cli.episodes_subs_directory)? {
        let dir_entry = if let Ok(dir_entry) = dir_entry {
            dir_entry
        } else {
            continue;
        };

        if let Ok(movie_file) = MovieFile::new(dir_entry.path()) {
            movie_files.push(movie_file);
            continue;
        };

        if let Ok(subtitle_file) = SubtitleFile::new(dir_entry.path()) {
            subtitle_files.push(subtitle_file);
        };
    }

    if !cli.ignore_number_difference && movie_files.len() != subtitle_files.len() {
        return Err(anyhow!(
            "Total movie files are not the same as total subtitle files. Movies: {}, Subtitles: {}",
            movie_files.len(),
            subtitle_files.len(),
        ));
    }

    for movie_file in movie_files.iter_mut() {
        if movie_file.is_matched() {
            continue;
        }
        for subtitle_file in subtitle_files.iter_mut() {
            if subtitle_file.is_renamed() {
                continue;
            }
            if let Err(SubtitleFileError::FileSystem(err)) =
                subtitle_file.rename_using_movie_file(movie_file)
            {
                return Err(anyhow!(err));
            } else {
                println!("Renamed subtitle file {}", subtitle_file);
                break;
            }
        }
    }

    Ok(())
}
