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

    /// Whether to get a summary of renamed and non-renamed subtitle files after rename completes.
    #[clap(short, long)]
    summarize: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    simple_logger::init()?;

    let mut movie_files = Vec::new();
    let mut subtitle_files = Vec::new();

    for dir_entry in fs::read_dir(cli.episodes_subs_directory)? {
        let dir_entry = match dir_entry {
            Ok(dir_entry) => dir_entry,
            Err(err) => {
                log::error!("Error reading a directory entry: {}", err);
                continue;
            }
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

    let mut renamed_subtitle_files = Vec::new();

    // keeping track of what subtitle file to remove from the vec after being renamed for efficiency
    let mut subtitle_file_index_to_remove: Option<usize> = None;

    movie_files.iter().for_each(|movie_file| {
        subtitle_files
            .iter()
            .enumerate()
            .any(|(index, subtitle_file)| {
                if let Err(err) = subtitle_file.rename_using_movie_file(movie_file) {
                    match err {
                        SubtitleFileError::FileSystem(err) => {
                            log::error!("{}", err);
                            log::warn!("Skipping errored file: '{}'", subtitle_file);
                        }
                        SubtitleFileError::AlreadyRenamed => {
                            log::warn!("Skipping already renamed file: '{}'", subtitle_file)
                        }
                        _ => {}
                    }
                    false
                } else {
                    println!("{} Renamed subtitle file '{}'", "->".green(), subtitle_file);
                    subtitle_file_index_to_remove = Some(index);
                    true
                }
            });

        if let Some(index) = subtitle_file_index_to_remove {
            let subtitle_file = subtitle_files.swap_remove(index);
            if cli.summarize {
                renamed_subtitle_files.push(subtitle_file);
            }
            subtitle_file_index_to_remove = None;
        }
    });

    if cli.summarize {
        println!("\n-------------- SUMMARY --------------");
        println!("{}", ":: Renamed subtitle files".blue());
        if renamed_subtitle_files.is_empty() {
            println!("Nothing.");
        } else {
            for sub in renamed_subtitle_files {
                println!("- {}", format!("{}", sub).green());
            }
        }

        println!("\n{}", ":: Non renamed subtitle files".blue());
        if subtitle_files.is_empty() {
            println!("Nothing.");
        } else {
            for sub in &subtitle_files {
                println!("- {}", format!("{}", sub).red());
            }
        }
    }

    println!(
        "\n{}",
        format!(
            "Renamed subtitle files : {}, Non-renamed subtitle files: {}",
            format!("{}", subtitle_files_before_rename - subtitle_files.len()).green(),
            format!("{}", subtitle_files.len()).red()
        )
        .blue()
    );

    Ok(())
}
