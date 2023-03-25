mod add;
mod env;
mod get;
mod remove;
mod server;

use anyhow::Result;
use clap::{Parser, Subcommand};

use add::AddCommand;
use get::GetCommand;
use remove::RemoveCommand;
use server::ServerCommand;

#[derive(Subcommand)]
pub enum Commands {
    Add(AddCommand),
    Get(GetCommand),
    Remove(RemoveCommand),
    Server(ServerCommand),
}

#[derive(Parser)]
#[command(name = "keycrab", version = "0.1", about = "A local password manager")]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.commands {
            Commands::Add(command) => command.execute().await,
            Commands::Get(command) => command.execute().await,
            Commands::Remove(command) => command.execute().await,
            Commands::Server(command) => command.execute().await,
        }
    }
}
