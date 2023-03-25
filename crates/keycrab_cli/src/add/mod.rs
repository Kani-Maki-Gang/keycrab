use std::env::var;

use anyhow::Result;
use clap::Args;
use keycrab_core::{connect::new_connection, machine_users::MachineUser, passwords::Password};
use keycrab_crypt::{gpg::GpgProxy, traits::CryptoProvider};

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

    #[arg(
        short = 'f',
        long = "fingerprint",
        help = "Public key fingerprint. Can also be set using the KEYCRAB_FINGERPRINT environment variable."
    )]
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
        let proxy = GpgProxy::new(machine_user.name.clone(), fingerprint);
        let encrypted_pass = proxy.encrypt(self.password)?;
        Password::insert(
            &mut conn,
            &machine_user.id,
            &self.domain,
            &self.username,
            &encrypted_pass,
        )
        .await?;

        Ok(())
    }
}
