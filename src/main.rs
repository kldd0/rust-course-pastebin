use std::io::{Read, Write};

use clap::Parser;
use service::Service;

mod cli;
mod service;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let service = Service::new(args.data_dir)?;
    match args.command {
        cli::Commands::Create { file: path } => {
            let mut file = std::fs::File::open(path)?;
            let mut body = String::new();
            file.read_to_string(&mut body)?;

            let id = service.create(body)?;
            println!("{}", id);
        }
        cli::Commands::Read { id } => {
            let body = service.read(&id)?;
            std::io::stdout().write_all(body.as_bytes())?;
        }
    }
    Ok(())
}
