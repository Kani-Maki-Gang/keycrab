use std::sync::Arc;

use axum::{routing::get, Router, Json, Extension, extract::Query};
use keycrab_crypt::{gpg::GpgProxy, traits::CryptoProvider};
use crate::{responses::PasswordResponse, layers::database::DatabaseLayer};
use tracing::info;

async fn password_get(Extension(database): Extension<Arc<DatabaseLayer>>) -> Json<PasswordResponse> {
    info!("mock password, database: {:?}", database);
    Json(PasswordResponse {
        username: "mock_user".to_owned(),
        password: "mock_password".to_owned(),
    })
}

async fn get_domain_credentials(Extension(database): Extension<Arc<DatabaseLayer>>, domain: Query<String>) -> Json<PasswordResponse> {
    info!("Received request: {:?}", domain);
    let credentials = database.get_password(domain.to_string())
        .await
        .unwrap();

    let crypto_provider = GpgProxy::new("kozlov".to_string());

    Json(PasswordResponse {
        username: credentials.user_id,
        password: crypto_provider.decrypt(credentials.password).unwrap()
    })
}

pub fn router() -> Router {
    info!("registering the password routes");
    Router::new()
        .route("/password", get(password_get))
        .route("/domain", get(get_domain_credentials))
}
