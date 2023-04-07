#![allow(non_snake_case)]

use dioxus::prelude::*;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Serialize, Deserialize)]
struct TabQueryArgs {
    active: bool,
}

#[derive(Serialize, Deserialize)]
struct Tab {
    id: Option<i32>,
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
    fn query(s: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "scripting"])]
    fn executeScript(s: JsValue) -> Result<(), JsValue>;
}

fn main() {
    dioxus_web::launch(App);
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
            files: vec!["sample.js".to_string()],
        };
        let value = serde_wasm_bindgen::to_value(&args).unwrap();
        executeScript(value)?;
    }

    Ok(())
}

fn Counter(cx: Scope) -> Element {
    let count = use_state(&cx, || 0);

    cx.render(rsx! {
        button {
            class: "bg-current",
            onclick: move |_| count.set(count + 1),
            "Reset {count}"
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
