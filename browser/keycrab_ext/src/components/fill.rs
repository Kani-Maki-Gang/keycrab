use anyhow::{anyhow, Result};
use dioxus::prelude::*;
use js_sys::Promise;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::models::{domains::DomainInfo, message::SendMessageArgs};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(catch, js_namespace = ["browser", "tabs"])]
    fn sendMessage(tabId: i32, args: JsValue) -> Result<JsValue, JsValue>;
}

fn send_fill_args(username: &str, password: &str) -> Result<JsValue> {
    let args = SendMessageArgs::new("fill", username, password);

    to_value(&args).map_err(|_| anyhow!("unable to create send message arguments"))
}

async fn send_fill_command(tab_id: i32, args: JsValue) -> Result<()> {
    let send_msg: Promise = sendMessage(tab_id, args)
        .map_err(|_| anyhow!("unable to create send message promise"))?
        .into();

    JsFuture::from(send_msg)
        .await
        .map_err(|_| anyhow!("send 'fill' message to active tab failed"))?;

    Ok(())
}

async fn on_click(tab_id: i32, args: JsValue) {
    if let Err(e) = send_fill_command(tab_id, args).await {
        log(&format!("{e:?}"));
    }
}

#[derive(Props)]
pub struct FillBtnProps<'a> {
    domain: &'a DomainInfo,
}

pub fn FillBtn<'a>(cx: Scope<'a, FillBtnProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        button {
            class: "rounded-full bg-red-700 text-stone-300 w-full drop-shadow",
            onclick: move |_| {
                let tab_id = cx.props.domain.tab_id;
                let username = &cx.props.domain.username;
                let password = &cx.props.domain.password;

                let Ok(args) = send_fill_args(username, password) else {
                    return;
                };

                use_future(cx, (), |_| async move {
                    on_click(tab_id, args).await
                }).value();
            },
            "Fill"
        }
    })
}
