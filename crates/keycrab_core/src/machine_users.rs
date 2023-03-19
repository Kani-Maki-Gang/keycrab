use anyhow::{anyhow, Result};
use nix::unistd::{getuid, User as LinuxUser};
use sqlx::{query, query_as, FromRow, SqliteConnection, Connection};

const CREATE_TABLE_QUERY: &str = include_str!("../queries/machine_users/create.sql");
const INSERT_QUERY: &str = include_str!("../queries/machine_users/insert.sql");
const DELETE_QUERY: &str = include_str!("../queries/machine_users/delete.sql");
const GET_QUERY: &str = include_str!("../queries/machine_users/get.sql");
const GET_BY_NAME_QUERY: &str = include_str!("../queries/machine_users/get_by_name.sql");

#[derive(FromRow)]
pub struct MachineUser {
    pub id: String,
    pub name: String,
    pub date_created: String,
    pub date_modified: String,
}

impl MachineUser {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<()> {
        query(CREATE_TABLE_QUERY)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn insert(conn: &mut SqliteConnection, id: &str, name: &str) -> Result<Self> {
        let mut transaction = conn.begin().await?;

        query(INSERT_QUERY)
            .bind(id)
            .bind(name)
            .execute(&mut transaction)
            .await
            .map_err(|e| anyhow!(e))?;

        let machine_user = Self::get(&mut transaction, id).await?;
        transaction.commit().await?;

        Ok(machine_user)
    }

    pub async fn delete(conn: &mut SqliteConnection, id: &str) -> Result<()> {
        query(DELETE_QUERY)
            .bind(id)
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn get(conn: &mut SqliteConnection, id: &str) -> Result<Self> {
        query_as::<_, Self>(GET_QUERY)
            .bind(id)
            .fetch_one(conn)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_by_name(conn: &mut SqliteConnection, name: &str) -> Result<Self> {
        query_as::<_, Self>(GET_BY_NAME_QUERY)
            .bind(name)
            .fetch_one(conn)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_from_sys(conn: &mut SqliteConnection) -> Result<Self> {
        let uid = getuid();
        let user = LinuxUser::from_uid(uid)?
            .ok_or_else(|| anyhow!("Couldn't retrieve info for current user"))?;

        let machine_user_id = uid.to_string();
        let machine_user = Self::get(conn, &machine_user_id).await;

        let machine_user = if machine_user.is_err() {
            Self::insert(conn, &machine_user_id, &user.name).await?
        } else {
            machine_user.unwrap()
        };

        Ok(machine_user)
    }
}
