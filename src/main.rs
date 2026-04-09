//! A simple wrapper around [ffmpeg](https://trac.ffmpeg.org/) that allows you to add metadata to
//! music media files.
//!
//! TODO: usage

#![warn(missing_docs)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use hj_meta::{
    cli::{Cli, Cmd},
    meta::meta,
};

use clap::Parser;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Meta(args) => meta(&args),

        Cmd::Split(args) => {
            let buffer = BufReader::new(File::open(&args.track_path)?);
            let mut tracks = Vec::new();
            for line in buffer.lines() {
                let track = Track::build(&line?, &args.source_file)?;
                tracks.push(track);
            }

            for track in tracks {
                println!(
                    "{}\n{}\n{}\n{}\n",
                    track.offset, track.title, track.output_dir, track.source_file
                );
            }

            Ok(())
        }
    }
}

pub struct Track {
    nose: String,
    tail: String,
    title: String,
    output_dir: String,
    source_file: String,
}

impl Track {
    pub fn build(line: &str, source_file: &impl AsRef<Path>) -> anyhow::Result<Track> {
        // TODO: make rgx global?
        let rgx = Regex::new(
            r"^\s*(?<nose>([0-1][0-9]|[2][0-3]:)?([0-5][0-9]:)?[0-5][0-9]\s*.{1}\s*(?<tail>[0-1][0-9]|[2][0-3]:)?([0-5][0-9]:)?[0-5][0-9])\s*.{1}\s*(?<title>.+)\s*$",
        )?;
        let (nose, tail, title) = rgx
            .captures_iter(&line)
            .find_map(|caps| {
                Some((
                    caps.name("nose")?.as_str().to_string(),
                    caps.name("tail")?.as_str().to_string(),
                    caps.name("title")?.as_str().to_string(),
                ))
            })
            // TODO:
            .ok_or_else(|| anyhow::anyhow!("TODO: "))?;
        Ok(Track {
            nose,
            tail,
            title,
            // TODO: make it configurable
            output_dir: "./output/".to_string(),
            source_file: source_file.as_ref().to_string_lossy().into(),
        })
    }

    pub fn command(&self) {
        // '/usr/bin/ffmpeg',
        // # '-nostdin',
        // '-y',
        // # 'loglevel',
        // # '8',
        // '-i',
        // to_be_splitted,
        // '-ss',
        // track['time 0'],
        // '-to',
        // track['time 1'],
        // # '-vn',
        // '-c',
        // 'copy',
    }
}
