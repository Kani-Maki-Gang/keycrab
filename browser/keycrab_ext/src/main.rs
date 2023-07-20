#![allow(non_snake_case)]

mod browser;
mod components;
mod models;

use browser::script;
use dioxus::prelude::*;

use crate::components::domains::Domains;

fn App(cx: Scope) -> Element {
    script::load(cx);

    cx.render(rsx! {
        div {
            class: "w-96 h-96 bg-slate-800 flex flex-row align-center",
            Domains {}
        }
    })
}

fn main() {
    dioxus_web::launch(App);
}
