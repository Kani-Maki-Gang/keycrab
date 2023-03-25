use std::env::var;

use anyhow::Result;
use clap::Args;
use keycrab_core::{connect::new_connection, machine_users::MachineUser, passwords::Password};

use crate::env::KEYCRAB_DATABASE;

#[derive(Args)]
#[command(about = "Removes a stored password")]
pub struct RemoveCommand {
    #[arg(
        short = 'd',
        long = "database",
        help = "The path to the keycrab database file. Can also be set using the KEYCRAB_DATABASE environment variable."
    )]
    database: Option<String>,

    #[arg(
        short = 'i',
        long = "id",
        help = "The id of the password in the provided database. Use a subcommand such as 'get' to find it if not known.",
        required = true
    )]
    id: String,
}

impl RemoveCommand {
    pub async fn execute(self) -> Result<()> {
        let database = self
            .database
            .map(Ok)
            .unwrap_or_else(|| var(KEYCRAB_DATABASE))?;

        let mut conn = new_connection(&database).await?;
        let machine_user = MachineUser::get_from_sys(&mut conn).await?;
        Password::delete(&mut conn, &self.id, &machine_user.id).await?;

        Ok(())
    }
}
