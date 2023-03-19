use anyhow::Result;
use clap::Args;
use std::env::var;

#[derive(Args)]
pub struct ServerCommand {
    #[arg(short = 'H', long = "host", help = "The server ip address to listen to.")]
    host: Option<String>,

    #[arg(short = 'P', long = "port", help = "The server port to listen to.")]
    port: Option<String>,

    #[arg(short = 'd', long = "database", help = "The path to the keycrab database file.")]
    database: Option<String>,
}

impl ServerCommand {
    pub async fn execute(self) -> Result<()> {
        let host = self
            .host
            .map(|x| Ok(x))
            .unwrap_or_else(|| var("KEYCRAB_HOST"))?;

        let port = self
            .port
            .map(|x| Ok(x))
            .unwrap_or_else(|| var("KEYCRAB_PORT"))?;

        let database = self
            .database
            .map(|x| Ok(x))
            .unwrap_or_else(|| var("KEYCRAB_DATABASE"))?;

        keycrab_server::start(&host, &port, &database).await
    }
}
