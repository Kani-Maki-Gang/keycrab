use std::{fs::File, path::Path};

use anyhow::{anyhow, Result};
use sqlx::{sqlite::SqlitePoolOptions, Connection, SqliteConnection, SqlitePool, migrate};

pub async fn new_connection(database: &str) -> Result<SqliteConnection> {
    SqliteConnection::connect(database)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn new_pool(database: &str) -> Result<SqlitePool> {
    let path = Path::new(database);
    if !path.is_file() {
        let _ = File::create(path)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(database)
        .await
        .map_err(|e| anyhow!(e))?;

    migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
