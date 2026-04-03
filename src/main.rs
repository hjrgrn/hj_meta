//! A simple wrapper around [ffmpeg](https://trac.ffmpeg.org/) that allows you to add metadata to
//! music media files.
//!
//! TODO: usage

#![warn(missing_docs)]

use clap::{Parser, Subcommand};

use hj_meta::meta::meta;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Meta {
            track_number,
            output_dir,
        } => meta(track_number, output_dir),
        Cmd::Split => Err(anyhow::anyhow!("Unimplemented")),
    }
}

#[derive(Parser, Debug)]
#[command(name = "HJMeta")]
#[command(author = "hjrgrn <187955624+hjrgrn@users.noreply.github.com>")]
// TODO:
#[command(about = "TODO", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Meta {
        #[arg(short, long, default_value = "true")]
        track_number: bool,

        #[arg(short, long, default_value = "true")]
        output_dir: bool,
    },

    Split,
}
