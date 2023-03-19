#[allow(dead_code)]
use anyhow::{anyhow, Result};
use sqlx::{query, query_as, FromRow, SqliteConnection};

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub date_created: String,
    pub date_modified: String,
}

impl User {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<()> {
        let create_query = include_str!("../../../queries/users/create.sql");
        query(create_query)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn insert(conn: &mut SqliteConnection, id: &str, name: &str) -> Result<User> {
        let insert_query = include_str!("../../../queries/users/insert.sql");
        query_as::<_, User>(insert_query)
            .bind(&id)
            .bind(&name)
            .fetch_one(conn)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn delete(conn: &mut SqliteConnection, id: &str) -> Result<()> {
        let delete_query = include_str!("../../../queries/users/delete.sql");
        query(delete_query)
            .bind(id)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_by_name(conn: &mut SqliteConnection, name: &str) -> Result<User> {
        let get_by_name_query = include_str!("../../../queries/users/get_by_name.sql");
        query_as::<_, User>(get_by_name_query)
            .bind(name)
            .fetch_one(conn)
            .await
            .map_err(|e| anyhow!(e))
    }
}
