use anyhow::{anyhow, Result};
use sqlx::{query, query_as, FromRow, SqliteConnection};

const CREATE_TABLE_QUERY: &str = include_str!("../queries/passwords/create.sql");
const INSERT_QUERY: &str = include_str!("../queries/passwords/insert.sql");
const DELETE_QUERY: &str = include_str!("../queries/passwords/delete.sql");
const GET_BY_USER_ID_QUERY: &str = include_str!("../queries/passwords/get_by_user_id.sql");
const GET_BY_DOMAIN_QUERY: &str = include_str!("../queries/passwords/get_by_domain.sql");
const SEARCH_DOMAINS: &str = include_str!("../queries/passwords/search_domains.sql");

#[derive(FromRow)]
pub struct Password {
    pub rowid: i32,
    pub machine_user_id: String,
    pub domain: String,
    pub username: String,
    pub password: String,
    pub date_created: String,
    pub date_modified: String,
}

impl Password {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<()> {
        query(CREATE_TABLE_QUERY)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn insert(
        conn: &mut SqliteConnection,
        machine_user_id: &str,
        domain: &str,
        username: &str,
        password: &str,
    ) -> Result<()> {
        query(INSERT_QUERY)
            .bind(machine_user_id)
            .bind(domain)
            .bind(username)
            .bind(password)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn delete(
        conn: &mut SqliteConnection,
        rowid: &str,
        machine_user_id: &str,
    ) -> Result<()> {
        query(DELETE_QUERY)
            .bind(rowid)
            .bind(machine_user_id)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_by_machine_user_id(
        conn: &mut SqliteConnection,
        machine_user_id: &str,
    ) -> Result<Vec<Self>> {
        query_as::<_, Self>(GET_BY_USER_ID_QUERY)
            .bind(machine_user_id)
            .fetch_all(conn)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_by_domain(conn: &mut SqliteConnection, domain: &str) -> Result<Self> {
        query_as::<_, Self>(GET_BY_DOMAIN_QUERY)
            .bind(domain)
            .fetch_one(conn)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn search_domains(
        conn: &mut SqliteConnection,
        machine_user_id: &str,
        domain: &str,
    ) -> Result<Vec<Self>> {
        dbg!(&machine_user_id);
        dbg!(&domain);
        query_as::<_, Self>(SEARCH_DOMAINS)
            .bind(domain)
            .bind(machine_user_id)
            .fetch_all(conn)
            .await
            .map_err(|e| anyhow!(e))
    }
}
