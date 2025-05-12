use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[arg(long, default_value = ".")]
    pub data_dir: PathBuf,

    #[arg(default_value = "db.json")]
    pub state: PathBuf,

    #[arg(long, short)]
    pub username: Option<String>,

    #[arg(long, short)]
    pub password: Option<String>,

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
    },

    /// registers a new user
    Register {},

    /// lists all pastes of the user
    List {},
}
