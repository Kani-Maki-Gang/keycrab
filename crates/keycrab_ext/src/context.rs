use serde::{Serialize, Deserialize};

#[derive(Clone, Default)]
pub struct SearchContext {
    pub query: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SettingsContext {
    pub host: String,
    pub port: String,
    pub tls: bool,
}
