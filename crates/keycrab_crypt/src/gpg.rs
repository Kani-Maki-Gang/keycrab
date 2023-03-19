use crate::traits::CryptoProvider;
use anyhow::{Result, anyhow};
use gpgme::{Context, Protocol};

pub struct GpgProxy {
    pub user: String
}

impl GpgProxy {
    pub fn new(user: String) -> GpgProxy {
        Self {
            user
        }
    }
}

impl CryptoProvider for GpgProxy {
    fn encrypt(&self, payload: String) -> Result<String> {
        let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
        ctx.set_armor(true);

        let recipients = vec![self.user.to_string()];
        let keys = if !recipients.is_empty() {
            ctx.find_keys(recipients)?
            .filter_map(|x| x.ok())
            .filter(|k| k.can_encrypt())
            .collect()
        } else {
            Vec::new()
        };

        let mut output = Vec::new();
        ctx.encrypt(&keys, payload, &mut output)
            .map(|r| format!("{:?}", r))
            .map_err(|e| anyhow!(e))
    }

    fn decrypt(&self, payload: String) -> Result<String> {
        let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
        let mut output = Vec::new();
        ctx.decrypt(payload, &mut output)
            .map(|r| format!("{:?}", r))
            .map_err(|e| anyhow!(e))
    }
}
