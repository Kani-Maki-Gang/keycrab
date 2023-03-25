use anyhow::Result;
use clap::Args;
use std::env::var;

use crate::env::{KEYCRAB_DATABASE, KEYCRAB_FINGERPRINT, KEYCRAB_HOST, KEYCRAB_PORT};

#[derive(Args)]
#[command(about = "Starts the password manager server")]
pub struct ServerCommand {
    #[arg(
        short = 'H',
        long = "host",
        help = "The server ip address to listen to. Can also be set using the KEYCRAB_HOST environment variable."
    )]
    host: Option<String>,

    #[arg(
        short = 'P',
        long = "port",
        help = "The server port to listen to. Can also be set using the KEYCRAB_PORT environment variable."
    )]
    port: Option<String>,

    #[arg(
        short = 'd',
        long = "database",
        help = "The path to the keycrab database file. Can also be set using the KEYCRAB_DATABASE environment variable."
    )]
    database: Option<String>,

    #[arg(
        short = 'f',
        long = "fingerprint",
        help = "Public key fingerprint. Can also be set using the KEYCRAB_FINGERPRINT environment variable."
    )]
    fingerprint: Option<String>,
}

impl ServerCommand {
    pub async fn execute(self) -> Result<()> {
        let host = self.host.map(Ok).unwrap_or_else(|| var(KEYCRAB_HOST))?;

        let port = self.port.map(Ok).unwrap_or_else(|| var(KEYCRAB_PORT))?;

        let database = self
            .database
            .map(Ok)
            .unwrap_or_else(|| var(KEYCRAB_DATABASE))?;

        let fingerprint = self
            .fingerprint
            .map(Ok)
            .unwrap_or_else(|| var(KEYCRAB_FINGERPRINT))?;

        keycrab_server::start(&host, &port, &database, &fingerprint).await
    }
}
