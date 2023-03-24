use std::env::var;

use anyhow::Result;
use clap::Args;

use crate::env::{KEYCRAB_DATABASE, KEYCRAB_FINGERPRINT};

#[derive(Args)]
#[command(about = "Add a new password")]
pub struct AddCommand {
    #[arg(
        short = 'd',
        long = "database",
        help = "The path to the keycrab database file. Can also be set using the KEYCRAB_DATABASE environment variable."
    )]
    database: Option<String>,

    #[arg(short = 'f', long = "fingerprint", help = "Public key fingerprint. Can also be set using the KEYCRAB_FINGERPRINT environment variable.")]
    fingerprint: Option<String>,

    #[arg(
        short = 'D',
        long = "domain",
        help = "The domain for the username/password pair.",
        required = true
    )]
    domain: String,

    #[arg(short = 'U', long = "username", required = true)]
    username: String,

    #[arg(short = 'P', long = "password", required = true)]
    password: String,
}

impl AddCommand {
    pub async fn execute(self) -> Result<()> {
        let _database = self
            .database
            .map(Ok)
            .unwrap_or_else(|| var(KEYCRAB_DATABASE))?;
        let _fingerprint = self
            .fingerprint
            .map(Ok)
            .unwrap_or_else(|| var(KEYCRAB_FINGERPRINT))?;

        Ok(())
    }
}
