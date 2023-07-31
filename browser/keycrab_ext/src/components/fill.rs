use dioxus::prelude::*;

pub fn FillBtn(cx: Scope) -> Element {
    cx.render(rsx! {
        button {
            class: "rounded-full bg-sky-800 text-stone-300 w-full",
            "Fill"
        }
    })
}
