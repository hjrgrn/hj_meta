//! Cli

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "HJMeta")]
#[command(author = "hjrgrn <187955624+hjrgrn@users.noreply.github.com>")]
// TODO:
#[command(about = "TODO", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Add metadata to an album.
    ///
    /// An album is supporsed to be a directory containing just the audio track of the album.
    // TODO:
    Meta(MetaArgs),

    Split,
}

#[derive(Debug, Args)]
pub struct MetaArgs {
    #[arg(short, long, default_value = "true")]
    pub track_number: bool,

    #[arg(short, long, default_value = "./output/")]
    pub output_dir: Option<String>,
}
