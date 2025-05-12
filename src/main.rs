use std::io::{Read, Write};

use clap::Parser;
use service::Service;
use state::State;

mod cli;
mod service;
mod state;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let state = State::load(&args.state)?;
    let mut service = Service::new(args.data_dir, state)?;
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
        cli::Commands::Register {} => {
            service.register_user(args.username.unwrap(), args.password.unwrap())?;
        }
        cli::Commands::List {} => {
            for id in service.list(args.username.unwrap(), args.password.unwrap())? {
                println!("{id}");
            }
        }
    }
    service.dump_state(&args.state)?;
    Ok(())
}
