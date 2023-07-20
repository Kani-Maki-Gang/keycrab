use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub id: i32,
    pub domain: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub credentials: Vec<Credential>,
}

pub struct DomainInfo {
    pub tab_id: i32,
    pub url: String,
    pub username: String,
    pub password: String,
}
