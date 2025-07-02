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
    #[wasm_bindgen(catch, js_namespace = ["browser", "tabs"])]
    fn query(s: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "tabs"])]
    fn sendMessage(tabId: i32, args: JsValue) -> Result<JsValue, JsValue>;
}

pub async fn query_active() -> Result<Tab> {
    let tab_query_args = to_value(&TabQueryArgs::new(true))
        .map_err(|_| anyhow!("unable to build tab query arguments"))?;

    let tab_query: Promise = query(tab_query_args)
        .map_err(|_| anyhow!("unable to query active tab"))?
        .into();

    let tab_query = JsFuture::from(tab_query)
        .await
        .map_err(|_| anyhow!("unable to query active tab"))?;

    let tabs: Vec<Tab> = from_value(tab_query)
        .map_err(|_| anyhow!("unable to read response from active tab future"))?;

    tabs.into_iter()
        .next()
        .ok_or_else(|| anyhow!("No active tab"))
}

pub async fn query_all() -> Result<Vec<Tab>> {
    let tab_query_args = to_value(&TabQueryArgs::new(true))
        .map_err(|_| anyhow!("unable to build tab query arguments"))?;

    let tab_query: Promise = query(tab_query_args)
        .map_err(|_| anyhow!("unable to query tabs"))?
        .into();

    let tab_query = JsFuture::from(tab_query)
        .await
        .map_err(|_| anyhow!("unable to query tabs"))?;

    from_value(tab_query).map_err(|_| anyhow!("unable to read response from tab future"))
}

pub fn send_fill_args(username: &str, password: &str) -> Result<JsValue> {
    let args = SendMessageArgs::new("fill", username, password);

    to_value(&args).map_err(|_| anyhow!("unable to create send message arguments"))
}

pub async fn send_fill_command(tab_id: i32, args: JsValue) -> Result<()> {
    let send_msg: Promise = sendMessage(tab_id, args)
        .map_err(|_| anyhow!("unable to create send message promise"))?
        .into();

    JsFuture::from(send_msg)
        .await
        .map_err(|_| anyhow!("send 'fill' message to active tab failed"))?;

    Ok(())
}
