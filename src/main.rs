//! A simple wrapper around [ffmpeg](https://trac.ffmpeg.org/) that allows you to add metadata to
//! music media files.
//!
//! TODO: usage

#![warn(missing_docs)]

use hj_meta::{
    cli::{Cli, Cmd},
    meta::meta,
};

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Meta(args) => meta(&args),

        Cmd::Split => Err(anyhow::anyhow!("Unimplemented")),
    }
}
