//! # `meta` subcommand
//!
//! Adds metadata to an album.
//! An album is expected to be a directory containing only the audio files of the album.
//!
//! ## Note
//!
//! If the `track_number` flag is provided, audio files must follow this format:
//! ```text
//! ^[0-9]+_.+$
//! ```
//! Avoid non-ASCII characters to prevent unexpected issues form `ffmpeg`.
// TODO: work on non-ascii chars

use anyhow;
use tempfile::TempDir;

use std::{
    fs::{self, create_dir, read_dir},
    io::{self, Write},
    process::Command,
};

use crate::cli::MetaArgs;

/// `meta` subcommand.
pub fn meta(meta_args: &MetaArgs) -> anyhow::Result<()> {
    let metadata = gather_metadata_from_user()?;

    add_metadata(meta_args, &metadata)
}

/// Struct that represents a metadata that can be added to the `ffmpeg` command.
#[derive(Debug, Clone)]
pub struct Metadata {
    /// Key of the metadata.
    key: String,
    /// Value of the metadata
    value: String,
}

impl Metadata {
    /// Build a new instance given the field and the data.
    pub fn new(field: &str, data: &str) -> Self {
        Self {
            key: field.to_string(),
            value: data.to_string(),
        }
    }

    /// Build a new instance by prompting the user for the data.
    pub fn prompt(s: &str) -> anyhow::Result<Self> {
        let key = s.to_string();
        print!("{key}: ");
        io::stdout().flush().map_err(|e| anyhow::anyhow!(e))?;
        let mut value = String::new();
        io::stdin().read_line(&mut value)?;
        Ok(Self { key, value })
    }

    /// Add the correct CLI arguments to a vector of CLI arguments.
    pub fn add_to_args(&self, args: &mut Vec<String>) {
        if !self.value.is_empty() {
            args.push("-metadata".to_string());
            args.push(format!("{}={}", self.key, self.value));
        }
    }
}

/// Builds and runs the command.
pub fn add_metadata(meta_args: &MetaArgs, metadata: &[Metadata]) -> anyhow::Result<()> {
    // TODO: fix this
    /// NOTE: Although this variable is not used directly, it must be retained to
    /// prevent the temporary directory from being dropped prematurely.
    #[allow(unused)]
    let tempdir: Option<TempDir>;
    #[allow(unused)]
    let output_dir: String = if let Some(od) = &meta_args.output_dir {
        tempdir = None;
        // TODO: maybe something more elegant
        format!("./{od}/")
    } else {
        let td = TempDir::new()?;
        let td_path = td
            .path()
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid UTF-8 encountered."))?
            .to_string();
        tempdir = Some(td);
        td_path
    };

    if tempdir.is_none() {
        create_dir(&output_dir)?;
    }

    run_cmd(&output_dir, metadata, tempdir)
}

/// Reduces cognitive complexity of `add_metadata`.
fn run_cmd(
    output_dir: &str,
    metadata: &[Metadata],
    tempdir: Option<TempDir>,
) -> anyhow::Result<()> {
    for entry in read_dir(".")? {
        // TODO: track number
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let old_path = entry
                .file_name()
                .into_string()
                .map_err(|e| anyhow::anyhow!(format!("{:?}", e)))?;
            let title = old_path.replace(":", "_");
            // TODO: fix this when implementing output_dir
            let new_path = format!("{}{}", &output_dir, title);

            let mut args = vec![
                "-i".to_string(),
                old_path.clone(),
                "-y".to_string(),
                "-c".to_string(),
                "copy".to_string(),
            ];

            for entry in metadata.iter() {
                entry.add_to_args(&mut args);
            }

            let title_metadata = Metadata::new("title", &title);
            title_metadata.add_to_args(&mut args);

            args.push(new_path.clone());

            let output = Command::new("/usr/bin/ffmpeg").args(args).output()?;
            if !output.status.success() {
                eprintln!("Could not write track {title}.");
            }
            if tempdir.is_some() {
                if fs::copy(new_path, old_path).is_err() {
                    eprintln!("Could not write track {title}.");
                }
            }
        }
    }
    Ok(())
}

/// Prompt the user for the metadata "artist", "album" and "genre".
// TODO: configurable fields from the CLI
pub fn gather_metadata_from_user() -> anyhow::Result<[Metadata; 3]> {
    Ok([
        Metadata::prompt("artist")?,
        Metadata::prompt("album")?,
        Metadata::prompt("genre")?,
    ])
}
