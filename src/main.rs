//! A script that allows you to add metadata to music media files.
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
    // TODO: duplication, maybe a struct
    print!("Author: ");
    io::stdout().flush().map_err(|e| anyhow::anyhow!(e))?;
    let mut author = String::new();
    io::stdin().read_line(&mut author)?;
    print!("Album: ");
    io::stdout().flush().map_err(|e| anyhow::anyhow!(e))?;
    let mut album = String::new();
    io::stdin().read_line(&mut album)?;
    print!("Genre: ");
    io::stdout().flush().map_err(|e| anyhow::anyhow!(e))?;
    let mut genre = String::new();
    io::stdin().read_line(&mut genre)?;

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
            args.push("-metadata".to_string());
            args.push(format!("title={title}"));
            // TODO: duplication, maybe useless memcopy
            if !author.is_empty() {
                args.push("-metadata".to_string());
                args.push(format!("author={author}"));
            }
            if !album.is_empty() {
                args.push("-metadata".to_string());
                args.push(format!("album={album}"));
            }
            if !genre.is_empty() {
                args.push("-metadata".to_string());
                args.push(format!("genre={genre}"));
            }
            if cli.output_dir {
                args.push(new_path);
            } else {
                args.push(title);
            }

            // FROMHERE: command
            let output = Command::new("/usr/bin/ffmpeg").args(args).output()?;
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(name = "HJMeta")]
#[command(author = "hjrgrn <187955624+hjrgrn@users.noreply.github.com>")]
#[command(about = "TODO", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "true")]
    track_number: bool,

    #[arg(short, long, default_value = "true")]
    output_dir: bool,
}
