//! CLI related utils.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// CLI.
#[derive(Parser, Debug)]
#[command(name = "HJMeta")]
#[command(author = "hjrgrn <187955624+hjrgrn@users.noreply.github.com>")]
// TODO:
#[command(about = "TODO", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

/// Subcommands.
#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// # `meta` subcommand
    ///
    /// Adds metadata to an album.
    /// An album is expected to be a directory containing only the audio files of the album.
    ///
    /// ## Note
    ///
    /// If the `track_number` flag is provided, audio files must follow this format:
    /// ```text
    /// ^[0-9]+_.+$
    /// ```
    /// Avoid non-ASCII characters to prevent unexpected issues.
    Meta(MetaArgs),

    /// # `split` subcommand
    ///
    /// Split a "full album" file into multiple tracks.
    /// A track list with each track's offset must be provided, and it must follow this format:
    /// ```text
    /// # tracklist.txt
    ///
    /// 00:00-02:38 - Oubliette
    /// 02:38-07:09 - Requiem
    /// 07:09-12:40 - Inhert
    /// 12:40-16:48 - Disfigured
    /// 16:48-20:48 - Multitude
    /// 20:48-25:07 - Ruins
    /// 25:07-29:16 - March
    /// 29:16-33:00 - Abattoir
    /// 33:00-36:30 - Feral
    /// 36:30-41:45 - Excalibur
    /// ```
    ///
    /// The name of the original file must be provided with its extension. For example:
    /// ```text
    /// Dan Terminus - Last Call For All Passengers.m4a
    /// ```
    ///
    /// ## Note
    ///
    /// Avoid non-ASCII characters in the name of the tracks to prevent unexpected
    /// issues.
    Split(SplitArgs),
}

/// CLI Arguments that can be passed to the `meta` subcommand.
#[derive(Debug, Args)]
pub struct MetaArgs {
    /// Adds track numbers to the metadata.
    ///
    /// For this feature to work correctly, audio file names must follow this format:
    /// ```text
    /// ^[0-9]+_.+$
    /// ```
    #[arg(short, long, default_value = "true")]
    pub track_number: bool,

    /// Saves the edited files to the specified directory.
    ///
    /// If this flag is not provided, the original files will be deleted.
    #[arg(short, long)]
    pub output_dir: Option<String>,
}

/// CLI Arguments that can be passed to the `split` subcommand.
#[derive(Debug, Args)]
pub struct SplitArgs {
    /// Path to the tracklist file.
    #[arg(short, long)]
    pub tracklist_path: PathBuf,

    /// Path to the album that will be splitted into multiple tracks.
    #[arg(short, long)]
    pub source_file: PathBuf,

    /// Directory where the resulting tracks will be saved, defaults to
    /// `./output_dir`.
    // TODO: maybe change this and the other output_dir
    #[arg(short, long, default_value = "./output_dir/")]
    pub output_dir: PathBuf,

    /// Prompt the user for metadata to be added to each track, defaults to
    /// `true`.
    #[arg(short, long, default_value = "true")]
    pub metadata: bool,
}
