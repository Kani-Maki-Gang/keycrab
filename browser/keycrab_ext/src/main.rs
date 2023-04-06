#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
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
    cx.render(rsx! {
        Counter{},
    })
}
