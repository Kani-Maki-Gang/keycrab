use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordResponse {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordCreateResponse {
    pub ok: bool,
}
