use dioxus::prelude::*;

pub fn DomainsLoading(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full text-gray-300 flex flex-row content-center justify-center p-8",
            svg {
                class: "bg-red-700 animate-spin h-5 w-5",
                "viewbox": "0 0 24 24",
            }
            div {
                class: "ml-4",
                "Loading domains..."
            }
        }
    })
}
