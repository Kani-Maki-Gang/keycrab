use anyhow::{Result, anyhow};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["navigator", "clipboard"], js_name = "writeText")]
    fn clipboard_write_text(value: String) -> Promise;
}

pub async fn write_text(value: String) -> Result<()> {
    JsFuture::from(clipboard_write_text(value))
        .await
        .map_err(|_| anyhow!("error while handling clipboard future"))?;
    Ok(())
}
