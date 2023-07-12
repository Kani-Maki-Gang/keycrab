#![allow(non_snake_case)]

use dioxus::prelude::*;
use js_sys::Promise;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

const BUTTON_CLASS: &str = "p-3 border border-black text-2xl uppercase rounded-full rounded-br-none outline-none shadow-lg hover:shadow-xl hover:rounded-full duration-200";

#[derive(Serialize, Deserialize)]
struct Credential {
    id: i32,
    domain: String,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SearchResponse {
    credentials: Vec<Credential>,
}

#[derive(Serialize, Deserialize)]
struct  SendMessageArgs {
    command: String,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct TabQueryArgs {
    active: bool,
}

#[derive(Serialize, Deserialize)]
struct Tab {
    id: Option<i32>,
    url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ExecuteScriptArgs {
    target: InjectionTarget,
    files: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct InjectionTarget {
    #[serde(rename = "tabId")]
    tab_id: i32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = document, js_name = createTextNode)]
    fn create_text_node(s: &str);

    #[wasm_bindgen(catch, js_namespace = ["browser", "tabs"])]
    fn sendMessage(tabId: i32, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "tabs"])]
    fn query(s: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "scripting"])]
    fn executeScript(s: JsValue) -> Result<(), JsValue>;
}

fn main() {
    dioxus_web::launch(App);
}

async fn find_active_tab() -> Result<Tab, JsValue> {
    let tab_query_args = TabQueryArgs { active: true };
    let tab_query: Promise = query(to_value(&tab_query_args)?)?.into();
    let tab_query = JsFuture::from(tab_query).await?;
    let tabs: Vec<Tab> = from_value(tab_query)?;
    tabs.into_iter().next().ok_or(JsValue::from_str("No active tab"))
}

async fn load_script() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let tab_query_args = TabQueryArgs { active: true };

    let tab_query: Promise = query(to_value(&tab_query_args)?)?.into();
    let tab_query = JsFuture::from(tab_query).await?;
    let tabs: Vec<Tab> = from_value(tab_query)?;

    for tab in tabs {
        let Some(tab_id) = tab.id else { continue; };
        let args = ExecuteScriptArgs {
            target: InjectionTarget { tab_id },
            files: vec!["fill_form.js".to_string()],
        };
        let value = serde_wasm_bindgen::to_value(&args).unwrap();
        executeScript(value)?;
    }

    Ok(())
}

async fn send_command(command: String, username: String, password: String) -> Result<(), JsValue> {
    let tab = find_active_tab().await?;
    let send_message_args = SendMessageArgs {
        command,
        username,
        password,
    };
    let send_msg: Promise = sendMessage(
        tab.id.unwrap(),
        to_value(&send_message_args)?
        )?.into();

    JsFuture::from(send_msg).await?;
    Ok(())
}

async fn get_login_for_url(url: String) -> Result<SearchResponse> {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8000/domain/search")
        .query(&[("q", url)])
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

async fn get_login_for_current_tab() -> Result<()> {
    let tab = find_active_tab().await.map_err(|_| anyhow::anyhow!("No active tab"))?;
    let url = tab.url.unwrap();
    let credentials = get_login_for_url(url)
        .await?
        .credentials
        .into_iter()
        .next()
        .ok_or(anyhow::anyhow!("No credentials found"))?;

    send_command(
        credentials.username,
        credentials.password,
        "fill".to_string()
        ).await.map_err(|_| anyhow::anyhow!("Failed to send command"))
}

fn Counter(cx: Scope) -> Element {
    let count = use_state(&cx, || 0);

    cx.render(rsx! {
        button {
            class: BUTTON_CLASS,
            onclick: move |_|  {
                let _ = use_future(cx, (), |_| async move {
                    if let Err(e) = get_login_for_current_tab().await {
                        log(&format!("{e:?}"));
                    }
                }).value();
                count.set(count + 1);
            },
            h1 {
                class: "text-3xl font-bold underline",
                "Reset {count}"
            }
        }
    })
}

fn App(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        if let Err(e) = load_script().await {
            log(&format!("{e:?}"));
        }
    });
    let _ = future.value();
    cx.render(rsx! {
        Counter{},
    })
}
