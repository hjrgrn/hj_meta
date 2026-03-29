//! A script that allows you to add metadata to music media files.
//!
//! TODO: usage

#![warn(missing_docs)]

use clap::Parser;

fn main() {
    let _ = Cli::parse();
}

#[derive(Parser)]
#[command(name = "HJMeta")]
#[command(author = "hjrgrn <187955624+hjrgrn@users.noreply.github.com>")]
#[command(about = "TODO", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "true")]
    track_number: bool,
}
