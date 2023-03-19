pub struct Configuration {
    pub host: String,
    pub port: String,
    pub database_url: String,
}

impl Configuration {
    pub fn new(host: &str, port: &str, database_url: &str) -> Self {
        Self {
            host: host.to_owned(),
            port: port.to_owned(),
            database_url: database_url.to_owned(),
        }
    }
}
