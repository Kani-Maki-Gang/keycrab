use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordCreateRequest {
    pub domain: String,
    pub username: String,
    pub password: String,
}
