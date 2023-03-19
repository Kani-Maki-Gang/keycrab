mod server;
mod test;

use anyhow::Result;
use clap::{Parser, Subcommand};

use server::ServerCommand;
use test::TestCommand;

#[derive(Subcommand)]
pub enum Commands {
    Test(TestCommand),
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
            Commands::Test(command) => command.execute().await,
            Commands::Server(command) => command.execute().await,
        }
    }
}
