//! # `split` subcommand
//!
//! Split a "full album" file into multiple tracks.
//! A track list with each track's offset must be provided, and it must follow this format:
//! ```text
//! # tracklist.txt
//!
//! 00:00-02:38 - Oubliette
//! 02:38-07:09 - Requiem
//! 07:09-12:40 - Inhert
//! 12:40-16:48 - Disfigured
//! 16:48-20:48 - Multitude
//! 20:48-25:07 - Ruins
//! 25:07-29:16 - March
//! 29:16-33:00 - Abattoir
//! 33:00-36:30 - Feral
//! 36:30-41:45 - Excalibur
//! ```
//!
//! The name of the original file must be provided with its extension. For example:
//! ```text
//! Dan Terminus - Last Call For All Passengers.m4a
//! ```

use std::{
    env::set_current_dir,
    fs::{File, create_dir},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use regex::Regex;

use crate::{
    cli::{MetaArgs, SplitArgs},
    meta::meta,
};

pub fn split(args: &SplitArgs) -> anyhow::Result<()> {
    let buffer = BufReader::new(File::open(&args.tracklist_path)?);
    let mut tracks = Vec::new();

    let rgx = Regex::new(r"^.+(?<ext>\..+)$")?;
    let ext = rgx
        .captures_iter(
            &args
                .source_file
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("TODO:"))?,
        )
        .find_map(|caps| Some(caps.name("ext")?.as_str().to_string()))
        .ok_or_else(|| anyhow::anyhow!("TODO"))?;

    create_dir(&args.output_dir)?;

    let mut track_number = 1;
    for line in buffer.lines() {
        let track = Track::build(
            &line?,
            &args.source_file,
            args.output_dir.clone(),
            &ext,
            track_number,
        )?;
        tracks.push(track);
        track_number += 1;
    }
    for track in tracks {
        track.command()?;
    }

    if args.metadata {
        set_current_dir(&args.output_dir)?;
        let meta_args = MetaArgs {
            track_number: true,
            output_dir: None,
        };
        meta(&meta_args)?;
    }

    Ok(())
}

/// A track that will be obtained from the original file.
pub struct Track {
    /// Beginning of the track.
    nose: String,
    /// End of the track.
    tail: String,
    /// Title.
    title: String,
    /// Directory where the track will be saved.
    // TODO: `String` or `PathBuf`
    output_dir: PathBuf,
    /// Original file that will be splitted.
    source_file: String,
    /// Extension of the track.
    ext: String,
    /// Index of the track inside the album.
    track_number: u8,
}

impl Track {
    /// Build a track.
    // TODO: `&impl AsRef<Path>` or `PathBuf`
    pub fn build(
        line: &str,
        source_file: &impl AsRef<Path>,
        output_dir: PathBuf,
        ext: &str,
        track_number: u8,
    ) -> anyhow::Result<Track> {
        // TODO: make rgx global?
        let rgx = Regex::new(
            r"^\s*((?<nose>([0-1][0-9]|[2][0-3]:)?([0-5][0-9]:)?[0-5][0-9])\s*.{1}\s*(?<tail>([0-1][0-9]|[2][0-3]:)?([0-5][0-9]:)?[0-5][0-9]))\s*.{1}\s*(?<title>.+)\s*$",
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
            output_dir,
            source_file: source_file.as_ref().to_string_lossy().into(),
            ext: ext.to_string(),
            track_number,
        })
    }

    /// Build the `ffmpeg` command that will be called.
    pub fn command(&self) -> io::Result<Output> {
        let track_number = if self.track_number < 10 {
            format!("0{}_", self.track_number)
        } else {
            format!("{}_", self.track_number)
        };
        Command::new("/usr/bin/ffmpeg")
            .args([
                "-y",
                "-i",
                &self.source_file,
                "-ss",
                &self.nose,
                "-to",
                &self.tail,
                "-c",
                "copy",
                &self
                    .output_dir
                    .join(&format!("{}{}{}", track_number, self.title, self.ext))
                    .to_string_lossy()
                    .to_string(),
            ])
            .output()
    }
}
