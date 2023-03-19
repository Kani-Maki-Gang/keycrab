use anyhow::Result;
use clap::Args;
use keycrab_core::connect::{create_mock_table, simple_sqlite_connection};

#[derive(Args)]
pub struct TestCommand {
    #[arg(short = 'd', long = "database", help = "run a database test")]
    database: bool,
}

impl TestCommand {
    pub async fn execute(self) -> Result<()> {
        println!("Executing test subcommand");
        if self.database {
            let conn_string = "mock.db";
            std::fs::File::create(conn_string)?;
            println!("creating database connection");
            let mut conn = simple_sqlite_connection(conn_string).await?;
            println!("creating mock table");
            let _ = create_mock_table(&mut conn).await?;
            println!("success!");
        }

        Ok(())
    }
}
