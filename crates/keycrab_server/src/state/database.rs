use anyhow::{anyhow, Result};
use keycrab_core::connect::new_pool;
use sqlx::pool::PoolConnection;
use sqlx::{Sqlite, SqlitePool};

#[derive(Debug)]
pub struct DatabasePool {
    inner: SqlitePool,
}

impl DatabasePool {
    pub async fn new(database: &str) -> Result<Self> {
        new_pool(database).await.map(|inner| Self { inner })
    }

    pub async fn get(&self) -> Result<PoolConnection<Sqlite>> {
        self.inner.acquire().await.map_err(|e| anyhow!(e))
    }
}
