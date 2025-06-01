mod app;
mod domain;

<<<<<<< Updated upstream
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
            class: "w-[28rem] h-full bg-neutral-900",
            div {
                class: "w-full h-full flex flex-row align-center",
                Domains {}
            }
        }
    })
}

fn main() {
    dioxus_web::launch(App);
=======
use leptos::prelude::*;
use app::App;

fn main() {
    mount_to_body(App);
>>>>>>> Stashed changes
}
