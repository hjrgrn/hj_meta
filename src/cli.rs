use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "HJMeta")]
#[command(author = "hjrgrn <187955624+hjrgrn@users.noreply.github.com>")]
// TODO:
#[command(about = "TODO", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Add metadata to an album.
    Meta {
        #[arg(short, long, default_value = "true")]
        track_number: bool,

        #[arg(short, long, default_value = "true")]
        output_dir: bool,
    },

    Split,
}
