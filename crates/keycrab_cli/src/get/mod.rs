use std::env::var;
use std::fmt::Write;

use anyhow::Result;
use clap::Args;
use keycrab_core::{machine_users::MachineUser, connect::new_connection, passwords::Password};
use keycrab_crypt::{gpg::GpgProxy, traits::CryptoProvider};

use crate::env::{KEYCRAB_DATABASE, KEYCRAB_FINGERPRINT};

#[derive(Args)]
#[command(about = "Get a stored password")]
pub struct GetCommand {
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
        required = true,
    )]
    domain: String,
}

impl GetCommand {
    pub async fn execute(self) -> Result<()> {
        let database = self
            .database
            .map(Ok)
            .unwrap_or_else(|| var(KEYCRAB_DATABASE))?;

        let fingerprint = self
            .fingerprint
            .map(Ok)
            .unwrap_or_else(|| var(KEYCRAB_FINGERPRINT))?;

        let mut conn = new_connection(&database).await?;
        let machine_user = MachineUser::get_from_sys(&mut conn).await?;
        let passwords = Password::search_domains(&mut conn, &self.domain).await?;
        let proxy = GpgProxy::new(machine_user.name.clone(), fingerprint);

        for entry in passwords.into_iter() {
            let password = proxy.decrypt(entry.password)?;
            let mut message = String::new();
            writeln!(message, "{:<10}: {}", "domain", entry.domain)?;
            writeln!(message, "{:<10}: {}", "username", entry.username)?;
            writeln!(message, "{:<10}: {}", "password", password)?;
            println!("{message}");
        }

        Ok(())
    }
}
