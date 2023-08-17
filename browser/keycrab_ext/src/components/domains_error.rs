use anyhow::Error;
use dioxus::prelude::*;

#[derive(Props)]
pub struct DomainsErrorProps<'a> {
    error: &'a Error,
}

pub fn DomainsError<'a>(cx: Scope<'a, DomainsErrorProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "w-full text-gray-300 flex flex-row content-center justify-center p-8",
            "Failed to load information due to {cx.props.error}"
        }
    })
}
