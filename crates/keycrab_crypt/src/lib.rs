pub mod traits;
pub mod gpg;

use anyhow::Result;
use gpgme::{Context, KeyListMode, Protocol};


pub fn example_pgp() -> Result<()> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    ctx.set_key_list_mode(KeyListMode::LOCAL)?;
    let keys = ctx.find_keys(vec!["kozlov"])?;
    dbg!(keys);
    return Ok(());
}
