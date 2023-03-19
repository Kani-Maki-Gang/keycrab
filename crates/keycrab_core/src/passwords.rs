#[allow(dead_code)]
use anyhow::{anyhow, Result};
use sqlx::{query, query_as, FromRow, SqliteConnection, SqlitePool};

#[derive(FromRow)]
pub struct Password {
    pub id: String,
    pub domain: String,
    pub user_id: String,
    pub password: String,
    pub date_created: String,
    pub date_modified: String,
}

impl Password {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<()> {
        let create_query = include_str!("../../../queries/passwords/create.sql");
        query(create_query)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn insert(
        conn: &SqlitePool,
        id: &str,
        user_id: &str,
        domain: &str,
        password: &str,
    ) -> Result<()> {
        let insert_query = include_str!("../../../queries/passwords/insert.sql");
        query(insert_query)
            .bind(id)
            .bind(user_id)
            .bind(domain)
            .bind(password)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn delete(conn: &mut SqliteConnection, id: &str) -> Result<()> {
        let delete_query = include_str!("../../../queries/passwords/delete.sql");
        query(delete_query)
            .bind(id)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_by_user_id(
        conn: &mut SqliteConnection,
        user_id: &str,
    ) -> Result<Vec<Password>> {
        let get_by_user_id_query = include_str!("../../../queries/passwords/get_by_user_id.sql");
        query_as::<_, Password>(get_by_user_id_query)
            .bind(user_id)
            .fetch_all(conn)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_by_domain(
        conn: &SqlitePool,
        domain: &str
        ) -> Result<Password> {
        let get_by_domain_query = include_str!("../../../queries/passwords/get_by_domain.sql");
        query_as::<_, Password>(get_by_domain_query)
            .bind(domain)
            .fetch_one(conn)
            .await
            .map_err(|e| anyhow!(e))
    }

}
