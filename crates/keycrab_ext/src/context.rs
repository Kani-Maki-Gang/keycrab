use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct SearchContext {
    pub query: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsContext {
    pub host: String,
    pub port: String,
    pub tls: bool,
}

impl Default for SettingsContext {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: String::new(),
            tls: true,
        }
    }
}
