use anyhow::{Result, anyhow};
use js_sys::Promise;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::models::script::{ExecuteScriptArgs, InjectionTarget};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(catch, js_namespace = ["browser", "scripting"], js_name = "executeScript")]
    fn browser_execute_script(s: JsValue) -> Result<Promise, JsValue>;

    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name = "executeScript")]
    fn chrome_execute_script(s: JsValue) -> Promise;
}

pub async fn load_fill_form(tab_id: &i32) -> Result<()> {
    let args = ExecuteScriptArgs {
        target: InjectionTarget { tab_id: *tab_id },
        files: vec!["fill_form.js".to_string()],
    };
    let args = to_value(&args).unwrap();

    let promise = browser_execute_script(args.clone())
        .or_else(|_| Ok::<Promise, JsValue>(chrome_execute_script(args)))
        .map_err(|_| anyhow!("unable to call executeScript"))?;

    JsFuture::from(promise)
        .await
        .map_err(|_| anyhow!("unable to load fill_form.js script"))?;
    Ok(())
}
