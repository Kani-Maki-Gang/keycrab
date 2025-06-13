use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse};

pub struct ApplicationError(Error);

impl<E> From<E> for ApplicationError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        let body = format!("Keycrab server error: {}", self.0);
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
