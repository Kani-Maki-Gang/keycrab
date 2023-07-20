use dioxus::prelude::*;

use crate::{models::domains::DomainInfo, components::fill::FillBtn};

#[derive(Props)]
pub struct DomainCardProps<'a> {
    domain: &'a DomainInfo
}

pub fn DomainCard<'a>(cx: Scope<'a, DomainCardProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "align-center p-4",
            div {
                class: "bg-slate-600 rounded m-4",
                div {
                    class: "flex",
                    div {
                        "Domain:"
                    },
                    div {
                        class: "grow ml-4",
                        "{cx.props.domain.url}"
                    }
                },
                div {
                    class: "flex",
                    div {
                        "Username:"
                    },
                    div {
                        class: "grow ml-4",
                        "{cx.props.domain.username}"
                    }
                },
                div {
                    class: "mt-4",
                    FillBtn {}
                }
            }
        }
    })
}
