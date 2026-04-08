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

    // TODO:
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

/// TODO: comment
#[derive(Debug, Args)]
pub struct SplitArgs {
    #[arg(short, long)]
    pub track_path: PathBuf,
}
