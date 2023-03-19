mod server;

use anyhow::Result;
use clap::{Parser, Subcommand};

use server::ServerCommand;

#[derive(Subcommand)]
pub enum Commands {
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
            Commands::Server(command) => command.execute().await,
        }
    }
}
