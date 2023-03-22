use std::sync::Arc;

use crate::{
    requests::passwords::{PasswordCreateRequest, PasswordQuery, SearchQuery},
    responses::{
        errors::ApplicationError,
        passwords::{DomainSearchResult, PasswordCreateResponse, PasswordResponse},
    },
    state::ApplicationState,
};
use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use keycrab_core::passwords::Password;
use keycrab_crypt::{gpg::GpgProxy, traits::CryptoProvider};
use tracing::info;

async fn search_domain_credentials(
    State(state): State<Arc<ApplicationState>>,
    params: Query<SearchQuery>,
) -> Result<Json<DomainSearchResult>, ApplicationError> {
    let mut conn = state.pool.get().await?;
    let response = Password::search_domains(&mut conn, &params.q)
        .await
        .map(|c| {
            DomainSearchResult {
                credentials: c
                    .into_iter()
                    .map(|p| PasswordResponse {
                        username: p.username,
                        password: p.password,
                    })
                    .collect(),
            }
        })?;

    Ok(Json(response))
}

async fn get_domain_credentials(
    State(state): State<Arc<ApplicationState>>,
    params: Query<PasswordQuery>,
) -> Result<Json<PasswordResponse>, ApplicationError> {
    info!("Received request: {:?}", params);

    let mut conn = state.pool.get().await?;
    let credentials = Password::get_by_domain(&mut conn, &params.domain).await?;

    let crypto_provider = GpgProxy::new(
        state.machine_user.name.to_owned(),
        state.config.fingerprint.clone(),
    );

    let response = PasswordResponse {
        username: credentials.username,
        password: crypto_provider.decrypt(credentials.password)?,
    };

    Ok(Json(response))
}

async fn post_domain_credentials(
    State(state): State<Arc<ApplicationState>>,
    Json(request): Json<PasswordCreateRequest>,
) -> Result<Json<PasswordCreateResponse>, ApplicationError> {
    info!("Received request: {:?}", request);

    let mut conn = state.as_ref().pool.get().await?;
    let crypto_provider = GpgProxy::new(
        state.machine_user.name.to_owned(),
        state.config.fingerprint.clone(),
    );
    let encrypted_pass = crypto_provider.encrypt(request.password)?;
    Password::insert(
        &mut conn,
        &state.machine_user.id,
        &request.domain,
        &request.username,
        &encrypted_pass,
    )
    .await?;

    let response = PasswordCreateResponse {
        domain: request.domain,
        username: request.username,
        password: encrypted_pass,
    };

    Ok(Json(response))
}

pub fn router() -> Router<Arc<ApplicationState>> {
    info!("registering the password routes");
    Router::new()
        .route("/domain", get(get_domain_credentials))
        .route("/domain", post(post_domain_credentials))
        .route("/domain/search", get(search_domain_credentials))
}
