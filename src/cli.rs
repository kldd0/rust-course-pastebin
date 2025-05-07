use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[arg(default_value = ".")]
    pub data_dir: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// upload a new file
    Create {
        /// path to the file to upload
        #[arg(short, long)]
        file: PathBuf,
    },
    /// read the existing file
    Read {
        /// ID of the file to read
        id: uuid::Uuid,
    }
}
