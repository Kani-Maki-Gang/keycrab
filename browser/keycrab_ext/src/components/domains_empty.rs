use dioxus::prelude::*;

pub fn DomainsEmpty(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full text-gray-300 flex flex-row content-center justify-center p-8",
            "No saved credentials found."
        }
    })
}
