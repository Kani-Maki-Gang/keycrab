use anyhow::{anyhow, Result};
use dioxus::prelude::{use_future, Scope};
use wasm_bindgen::prelude::*;

use crate::models::script::{ExecuteScriptArgs, InjectionTarget};

use super::tab;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(catch, js_namespace = ["chrome", "scripting"])]
    fn executeScript(s: JsValue) -> Result<(), JsValue>;
}

async fn load_inner() -> Result<()> {
    console_error_panic_hook::set_once();

    let tabs = tab::query_all().await?;

    for tab in tabs {
        let Some(tab_id) = tab.id else { continue; };
        let args = ExecuteScriptArgs {
            target: InjectionTarget { tab_id },
            files: vec!["fill_form.js".to_string()],
        };
        let value = serde_wasm_bindgen::to_value(&args).unwrap();
        executeScript(value).map_err(|_| anyhow!("unable to load script"))?;
    }

    Ok(())
}

pub fn load(cx: Scope) {
    let future = use_future(cx, (), |_| async move {
        if let Err(e) = load_inner().await {
            log(&format!("{e:?}"));
        }
    });

    let _ = future.value();
}
