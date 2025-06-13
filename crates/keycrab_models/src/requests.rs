use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordQuery {
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordIdQuery {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCreateRequest {
    pub domain: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}
