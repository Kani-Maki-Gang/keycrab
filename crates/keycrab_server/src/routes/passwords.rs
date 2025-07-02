use std::sync::Arc;

use crate::{errors::ApplicationError, state::ApplicationState};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    routing::{delete, get, post},
    Json, Router,
};
use keycrab_core::passwords::Password;
use keycrab_crypt::{gpg::GpgProxy, traits::CryptoProvider};
use keycrab_models::{
    requests::{DecryptQuery, PasswordCreateRequest, PasswordIdQuery, SearchQuery},
    responses::{DomainInfo, DomainSearchResult, PasswordCreateResponse},
};
use tracing::info;

async fn search_domain_credentials(
    State(state): State<Arc<ApplicationState>>,
    params: Query<SearchQuery>,
) -> Result<Json<DomainSearchResult>, ApplicationError> {
    let mut conn = state.pool.get().await?;
    let response = Password::search_domains(&mut conn, &state.machine_user.id, &params.q)
        .await
        .map(|c| DomainSearchResult {
            credentials: c
                .into_iter()
                .flat_map(|p| {
                    Ok::<DomainInfo, anyhow::Error>(DomainInfo {
                        id: p.rowid,
                        domain: p.domain,
                        username: p.username,
                    })
                })
                .collect(),
        })?;

    Ok(Json(response))
}

async fn decrypt_domain_password(
    State(state): State<Arc<ApplicationState>>,
    params: Query<DecryptQuery>,
) -> Result<Json<String>, ApplicationError> {
    let mut conn = state.pool.get().await?;
    let domains = Password::get_by_domain(&mut conn, &params.domain).await?;

    if let Some(domain) = domains.into_iter().find(|x| x.username == params.username) {
        let proxy = GpgProxy::new(
            state.machine_user.name.clone(),
            state.config.fingerprint.clone(),
        );
        let password = proxy.decrypt(domain.password)?;
        Ok(Json(password))
    } else {
        Err(ApplicationError(anyhow!("entry not found")))
    }
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

async fn delete_domain_credentials(
    State(state): State<Arc<ApplicationState>>,
    Query(q): Query<PasswordIdQuery>,
) -> Result<(), ApplicationError> {
    info!("Received request: {:?}", q);

    let mut conn = state.pool.get().await?;
    Password::delete(&mut conn, &q.id, &state.machine_user.id).await?;

    Ok(())
}

pub fn router() -> Router<Arc<ApplicationState>> {
    info!("registering the password routes");
    Router::new()
        .route("/domain", post(post_domain_credentials))
        .route("/domain", delete(delete_domain_credentials))
        .route("/domain/search", get(search_domain_credentials))
        .route("/domain/decrypt", get(decrypt_domain_password))
}
