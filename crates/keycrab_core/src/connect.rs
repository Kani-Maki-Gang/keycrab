use anyhow::{anyhow, Result};
use sqlx::{sqlite::SqlitePoolOptions, Connection, SqliteConnection, SqlitePool, Executor};

pub async fn new_connection(database: &str) -> Result<SqliteConnection> {
    SqliteConnection::connect(database)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn new_pool(database: &str) -> Result<SqlitePool> {
    SqlitePoolOptions::new()
        .max_connections(20)
        .connect(database)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn simple_sqlite_connection(conn_string: &str) -> Result<SqliteConnection> {
    SqliteConnection::connect(conn_string)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn create_mock_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query("create table mock_table(id text primary key)")
        .execute(conn)
        .await
        .map(|x| {
            dbg!(x);
        })
        .map_err(|e| anyhow!(e))
}
