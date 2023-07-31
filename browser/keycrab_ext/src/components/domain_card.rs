use dioxus::prelude::*;

use crate::{models::domains::DomainInfo, components::fill::FillBtn};

#[derive(Props)]
pub struct DomainCardProps<'a> {
    domain: &'a DomainInfo
}

pub fn DomainCard<'a>(cx: Scope<'a, DomainCardProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "align-center",
            div {
                class: "bg-slate-600 rounded m-1 p-2",
                div {
                    class: "grid grid-cols-2",
                    div {
                        "Domain:"
                    },
                    div {
                        "{cx.props.domain.url}"
                    }
                },
                div {
                    class: "grid grid-cols-2",
                    div {
                        "Username:"
                    },
                    div {
                        class: "grow mt-4",
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
