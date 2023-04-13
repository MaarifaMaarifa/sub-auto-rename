# sub-auto-rename

A program that automatically renames subtitle files to their corresponding movie files names.

## Why this program exists

Modern video players such as vlc and mpv automatically pick up a subtitle file that has the same name as the movie that is being played. Sometimes you might have alot of videos that have their subtitle files with sightly different names which means you have to use your players settings to point which file to use or rename all the files manually which is tideous. This is where **sub-auto-rename** becomes handy.
The program will look through all videos and subtitle files in the directory, identify which subtitle file points to which movie file and automatically rename the subtitle files to the same names as their corresponding movies saving time if all of this work could be performed manually.

## How to use

Just dump all the videos and subtitles into one directory and give the program that directory path as a commandline option, sit back and wait for the magic to happen. You can also pass --help option to reveal full details of the available options.

### Examples
```shell
# Typical use
sub-auto-rename path/to/videos

# Ignoring the difference in the number of videos and subtitles in the provided directory
sub-auto-rename -i path/to/videos

# Getting help information
sub-auto-rename --help
```

## Installation

**sub-auto-rename** can be installed via the following shell commands assuming you have cargo, rustc and git set up on your machine. You can check the [guide](https://rustup.rs/) incase you're not setup.

```shell
git clone https://github.com/MaarifaMaarifa/sub-auto-rename
cd sub-auto-rename
cargo install --path .
```
