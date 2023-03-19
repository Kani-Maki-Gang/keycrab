mod layers;
mod responses;
mod routes;
mod requests;

use anyhow::{anyhow, Result};
use axum::{Extension, Router};
use keycrab_crypt::traits::CryptoProvider;
use keycrab_core::traits::IntoArc;
use routes::{passwords, users};
use tracing::info;
use layers::database::DatabaseLayer;

/// Define for each group of routes a router method and merge it here.
fn api_router() -> Router {
    users::router().merge(passwords::router())
}

pub async fn start(host: &str, port: &str, database: &str) -> Result<()> {
    tracing_subscriber::fmt::init();

    // initialize layers.
    let database_layer = DatabaseLayer::new(database).await?.into_arc();

    // initialize app.
    let app = api_router().layer(Extension(database_layer));

    // start server.
    let url = format!("{host}:{port}");

    info!("listening on {url}");
    axum::Server::bind(&url.parse()?)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!(e))
}
