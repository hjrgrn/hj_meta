//! A simple wrapper around [ffmpeg](https://trac.ffmpeg.org/) that allows you to add metadata to
//! music media files.
//!
//! TODO: usage

#![warn(missing_docs)]

use std::{
    fs::{create_dir, read_dir},
    io::{self, Write},
    process::Command,
};

use clap::Parser;

const OUTPUT_DIR: &str = "./output/";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if !cli.output_dir {
        return Err(anyhow::anyhow!("Unimplemented."));
    }

    // TODO: configurable fields
    let mut metadata = Vec::new();
    metadata.push(Metadata::prompt("author")?);
    metadata.push(Metadata::prompt("album")?);
    metadata.push(Metadata::prompt("genre")?);

    create_dir(OUTPUT_DIR)?;

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
            let new_path;
            if cli.output_dir {
                new_path = format!("{}{}", OUTPUT_DIR, title);
            } else {
                return Err(anyhow::anyhow!("Unimplemented."));
            }

            let mut args = Vec::new();
            args.push("-i".to_string());
            args.push(old_path);
            args.push("-y".to_string());
            args.push("-c".to_string());
            args.push("copy".to_string());

            for entry in metadata.iter() {
                entry.add_to_args(&mut args);
            }

            let title_metadata = Metadata::new("title", &title);
            title_metadata.add_to_args(&mut args);

            if cli.output_dir {
                args.push(new_path);
            } else {
                args.push(title);
            }

            let output = Command::new("/usr/bin/ffmpeg").args(args).output()?;
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(name = "HJMeta")]
#[command(author = "hjrgrn <187955624+hjrgrn@users.noreply.github.com>")]
// TODO:
#[command(about = "TODO", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "true")]
    track_number: bool,

    #[arg(short, long, default_value = "true")]
    output_dir: bool,
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
