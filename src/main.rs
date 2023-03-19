use anyhow::Result;
use clap::Parser;
use keycrab_cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    Cli::parse().execute().await
}
