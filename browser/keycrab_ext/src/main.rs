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
        body {
            class: "w-[28rem] h-full bg-slate-800",
            div {
                class: "w-full h-full flex flex-row align-center",
                Domains {}
            }
        }
    })
}

fn main() {
    dioxus_web::launch(App);
}
