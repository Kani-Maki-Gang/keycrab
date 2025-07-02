use anyhow::{Result, anyhow};
use js_sys::Promise;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::models::{
    message::SendMessageArgs,
    tab::{Tab, TabQueryArgs},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["browser", "tabs"], js_name = "query")]
    fn browser_query(s: JsValue) -> Result<Promise, JsValue>;

    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "query")]
    fn chrome_query(s: JsValue) -> Promise;

    #[wasm_bindgen(catch, js_namespace = ["browser", "tabs"], js_name = "sendMessage")]
    fn browser_send_message(tabId: i32, args: JsValue) -> Result<Promise, JsValue>;

    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "sendMessage")]
    fn chrome_send_message(tabId: i32, args: JsValue) -> Promise;
}

pub async fn get_current() -> Result<Tab> {
    let args = to_value(&TabQueryArgs::new(true, true))
        .map_err(|_| anyhow!("unable to build tab query arguments"))?;

    let promise = browser_query(args.clone())
        .or_else(|_| Ok::<Promise, JsValue>(chrome_query(args)))
        .map_err(|_| anyhow!("unable to call query function"))?;

    let tabs = JsFuture::from(promise)
        .await
        .map_err(|_| anyhow!("unable to retrieve tabs from promise"))?;

    let tab: Vec<Tab> = from_value(tabs).map_err(|e| anyhow!(e.to_string()))?;

    tab.into_iter()
        .next()
        .ok_or_else(|| anyhow!("no active tab found"))
}

pub async fn send_fill_command(tab_id: i32, username: &str, password: &str) -> Result<()> {
    let args = SendMessageArgs::new("fill", username, password);
    let args = to_value(&args).map_err(|_| anyhow!("unable to create send message arguments"))?;

    let promise = browser_send_message(tab_id, args.clone())
        .or_else(|_| Ok::<Promise, JsValue>(chrome_send_message(tab_id, args)))
        .map_err(|_| anyhow!("unable to call query function"))?;

    JsFuture::from(promise)
        .await
        .map_err(|_| anyhow!("send 'fill' message to active tab failed"))?;

    Ok(())
}
