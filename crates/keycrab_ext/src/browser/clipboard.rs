use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["navigator", "clipboard"])]
    fn writeText(value: String) -> Promise;
}

pub fn write_text(value: String) {
    let _ = writeText(value);
}
