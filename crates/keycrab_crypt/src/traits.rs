use anyhow::Result;

pub trait CryptoProvider {
    fn encrypt(&self, payload: String) -> Result<String>;
    fn decrypt(&self, payload: String) -> Result<String>;
}

