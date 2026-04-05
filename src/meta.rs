//! # meta subcommand
//!
//! Add metadata to an album.
// TODO:

use anyhow;

use std::{
    fs::{create_dir, read_dir},
    io::{self, Write},
    process::Command,
};

use crate::cli::MetaArgs;

pub fn meta(meta_args: &MetaArgs) -> anyhow::Result<()> {
    // TODO: fix this when implementing output_dir
    let output_dir: String = if let Some(od) = &meta_args.output_dir {
        od.clone()
    } else {
        return Err(anyhow::anyhow!("Unimplemented."));
    };

    // TODO: configurable fields
    // TODO: this will be passed as an argument to `meta`
    let metadata = [
        Metadata::prompt("author")?,
        Metadata::prompt("album")?,
        Metadata::prompt("genre")?,
    ];

    create_dir(output_dir)?;

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
            let new_path = if let Some(od) = &meta_args.output_dir {
                format!("{}{}", od, title)
            } else {
                return Err(anyhow::anyhow!("Unimplemented."));
            };

            let mut args = vec![
                "-i".to_string(),
                old_path,
                "-y".to_string(),
                "-c".to_string(),
                "copy".to_string(),
            ];

            for entry in metadata.iter() {
                entry.add_to_args(&mut args);
            }

            let title_metadata = Metadata::new("title", &title);
            title_metadata.add_to_args(&mut args);

            // TODO: fix this when implementing output_dir
            if meta_args.output_dir.is_some() {
                args.push(new_path);
            } else {
                args.push(title.clone());
            }

            let output = Command::new("/usr/bin/ffmpeg").args(args).output()?;
            if !output.status.success() {
                eprintln!("Could not write track {title}.");
            }
        }
    }

    Ok(())
}

// Struct that represents a metadata that can be added to the `ffmpeg` command.
struct Metadata {
    // Key of the metadata.
    key: String,
    // Value of the metadata
    value: String,
}

impl Metadata {
    // Build a new instance given the field and the data.
    fn new(field: &str, data: &str) -> Self {
        Self {
            key: field.to_string(),
            value: data.to_string(),
        }
    }

    // Build a new instance by prompting the user for the data.
    fn prompt(s: &str) -> anyhow::Result<Self> {
        let key = s.to_string();
        print!("{key}: ");
        io::stdout().flush().map_err(|e| anyhow::anyhow!(e))?;
        let mut value = String::new();
        io::stdin().read_line(&mut value)?;
        Ok(Self { key, value })
    }

    // Add the correct CLI arguments to a vector of CLI arguments.
    fn add_to_args(&self, args: &mut Vec<String>) {
        if !self.value.is_empty() {
            args.push("-metadata".to_string());
            args.push(format!("{}={}", self.key, self.value));
        }
    }
}
