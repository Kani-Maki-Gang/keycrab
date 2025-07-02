use anyhow::{Result, anyhow};
use leptos::{
    leptos_dom::logging::console_error,
    task::{spawn_local, spawn_local_scoped},
};
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

async fn load_fill_form_inner() -> Result<()> {
    let tabs = tab::query_all().await?;

    for tab in tabs {
        let Some(tab_id) = tab.id else {
            continue;
        };
        let args = ExecuteScriptArgs {
            target: InjectionTarget { tab_id },
            files: vec!["fill_form.js".to_string()],
        };
        let value = serde_wasm_bindgen::to_value(&args).unwrap();
        executeScript(value).map_err(|_| anyhow!("unable to load script"))?;
    }

    Ok(())
}

pub async fn load_fill_form() -> () {
    if let Err(error) = load_fill_form_inner().await {
        console_error(error.to_string().as_str());
    }
}
