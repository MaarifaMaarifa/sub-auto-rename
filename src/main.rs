use anyhow::{bail, Result};
use clap::Parser;
use colored::*;
use std::fs;
use std::path;
use sub_auto_rename::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The directory where there are all the episodes and
    /// and their corresponding subtitle files
    episodes_subs_directory: path::PathBuf,

    /// Extra movie extensions to include when checking movie files in a directory
    extra_movie_extensions: Option<Vec<String>>,

    /// Whether to ignore the difference in the number of files between subtitle files
    /// and episodes files as the default behaviour expects them to be of equal amount.
    #[clap(short, long)]
    ignore_number_difference: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    simple_logger::init()?;

    let mut movie_files = Vec::new();
    let mut subtitle_files = Vec::new();

    for dir_entry in fs::read_dir(cli.episodes_subs_directory)? {
        let dir_entry = if let Ok(dir_entry) = dir_entry {
            dir_entry
        } else {
            continue;
        };

        if let Some(movie_file) =
            MovieFile::new(dir_entry.path(), cli.extra_movie_extensions.as_ref())
        {
            movie_files.push(movie_file);
            continue;
        };

        if let Ok(subtitle_file) = SubtitleFile::try_from(dir_entry.path()) {
            subtitle_files.push(subtitle_file);
        };
    }

    if !cli.ignore_number_difference && movie_files.len() != subtitle_files.len() {
        bail!(
            "Total movie files are not the same as total subtitle files. Movies: {}, Subtitles: {}",
            movie_files.len(),
            subtitle_files.len(),
        );
    }

    let subtitle_files_before_rename = subtitle_files.len();

    // keeping track of what subtitle file to remove from the vec after being renamed for efficiency
    let mut subtitle_file_index_to_remove: Option<usize> = None;

    for movie_file in movie_files.iter() {
        for (index, subtitle_file) in subtitle_files.iter().enumerate() {
            if let Err(err) = subtitle_file.rename_using_movie_file(movie_file) {
                if let SubtitleFileError::FileSystem(err) = err {
                    log::error!("{}", err);
                    log::warn!("Skipping '{}' due to previous error", subtitle_file);
                }
            } else {
                println!("{} Renamed subtitle file '{}'", "->".green(), subtitle_file);
                subtitle_file_index_to_remove = Some(index);
                break;
            }
        }

        if let Some(index) = subtitle_file_index_to_remove {
            subtitle_files.swap_remove(index);
            subtitle_file_index_to_remove = None;
        }
    }

    println!(
        "{}",
        format!(
            "Total subtitle files renamed: {}",
            subtitle_files_before_rename - subtitle_files.len()
        )
        .blue()
    );

    Ok(())
}
