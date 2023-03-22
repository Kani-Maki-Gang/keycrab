mod requests;
mod responses;
mod routes;
mod state;

use anyhow::{anyhow, Result};
use axum::Router;
use keycrab_core::{machine_users::MachineUser, traits::IntoArc};
use routes::{machine_users, passwords};
use state::ApplicationState;
use std::sync::Arc;
use tracing::info;

use crate::state::{config::Configuration, database::DatabasePool};

/// Define for each group of routes a router method and merge it here.
fn api_router() -> Router<Arc<ApplicationState>> {
    machine_users::router().merge(passwords::router())
}

async fn api_state(
    host: &str,
    port: &str,
    database: &str,
    fingerprint: &str,
) -> Result<Arc<ApplicationState>> {
    let config = Configuration::new(host, port, database, fingerprint);
    let pool = DatabasePool::new(database).await?;
    let mut conn = pool.get().await?;
    let machine_user = MachineUser::get_from_sys(&mut conn).await?;
    Ok(ApplicationState::new(config, pool, machine_user).into_arc())
}

pub async fn start(host: &str, port: &str, database: &str, fingerprint: &str) -> Result<()> {
    tracing_subscriber::fmt::init();

    // initialize state.
    let state = api_state(host, port, database, fingerprint).await?;

    // initialize app.
    let app = api_router().with_state(state);

    // start server.
    let url = format!("{host}:{port}");

    info!("listening on {url}");
    axum::Server::bind(&url.parse()?)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!(e))
}
