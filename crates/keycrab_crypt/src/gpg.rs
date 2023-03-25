use crate::traits::CryptoProvider;
use anyhow::{anyhow, Result};
use gpgme::{Context, Protocol};

pub struct GpgProxy {
    pub user: String,
    pub fingerprint: String,
}

impl GpgProxy {
    pub fn new(user: String, fingerprint: String) -> GpgProxy {
        Self { user, fingerprint }
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
                .filter(|x| {
                    x.fingerprint()
                        .map(|y| y == self.fingerprint)
                        .unwrap_or_default()
                })
                .collect()
        } else {
            Vec::new()
        };

        let mut output = Vec::new();
        ctx.encrypt(&keys, payload, &mut output)?;

        String::from_utf8(output).map_err(|e| anyhow!(e))
    }

    fn decrypt(&self, payload: String) -> Result<String> {
        let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
        let mut output = Vec::new();
        ctx.decrypt(payload, &mut output)?;

        String::from_utf8(output).map_err(|e| anyhow!(e))
    }
}
