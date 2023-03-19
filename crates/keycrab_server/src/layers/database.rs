use std::fmt::Display;

use anyhow::{Result, anyhow};
use axum::extract::{FromRef, FromRequestParts};
use keycrab_core::connect::{new_pool, new_connection};
use keycrab_core::passwords::Password;
use sqlx::SqlitePool;

use crate::requests::PasswordCreateRequest;
use crate::responses::PasswordResponse;

#[derive(Debug)]
pub struct DatabaseLayer {
    pub pool: SqlitePool,
}

impl DatabaseLayer {
    pub async fn new(database: &str) -> Result<Self> {
        new_pool(database).await.map(|pool| Self { pool })
    }
    pub async fn create_password(&self, password: &PasswordCreateRequest) -> Result<()> {
        Password::insert(&self.pool,
                         "",
                         &password.username,
                         &password.domain,
                         &password.password
                         )
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_password(&self, domain: String) -> Result<Password> {
        Password::get_by_domain(&self.pool, &domain)
            .await
            .map_err(|e| anyhow!(e))
    }
}

impl Display for DatabaseLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
