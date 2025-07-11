use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct SearchContext {
    pub query: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct SettingsContext {
    pub host: String,
    pub port: String,
    pub tls: bool,
}

impl SettingsContext {
    pub fn new(host: String, port: String, tls: bool) -> Self {
        Self { host, port, tls }
    }

    pub fn base_url(&self) -> String {
        let protocol = if self.tls { "https" } else { "http" };
        format!("{protocol}://{}:{}", self.host, self.port)
    }
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
