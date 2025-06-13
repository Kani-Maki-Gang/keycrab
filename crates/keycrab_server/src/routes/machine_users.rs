use std::sync::Arc;

use axum::{extract::State, routing::get, Router};

use crate::{errors::ApplicationError, state::ApplicationState};

async fn get_user(State(_state): State<Arc<ApplicationState>>) -> Result<String, ApplicationError> {
    Ok("hello world".to_owned())
}

pub fn router() -> Router<Arc<ApplicationState>> {
    Router::new().route("/user", get(get_user))
}
