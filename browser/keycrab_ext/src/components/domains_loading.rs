use dioxus::prelude::*;

pub fn DomainsLoading(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-96 text-gray-300 flex flex-row content-center justify-center p-8",
            svg {
                class: "bg-indigo-500 animate-spin h-5 w-5",
                "viewbox": "0 0 24 24",
            }
            "Loading domains..."
        }
    })
}
