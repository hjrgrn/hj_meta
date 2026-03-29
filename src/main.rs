//! A script that allows you to add metadata to music media files.
//!
//! TODO: usage

#![warn(missing_docs)]

use std::fs::create_dir;

use clap::Parser;

const OUTPUT_DIR: &str = "./output";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if !cli.output_dir {
        return Err(anyhow::anyhow!("Unimplemented."));
    }

    create_dir(OUTPUT_DIR)?;

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
