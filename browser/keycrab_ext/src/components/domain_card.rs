use dioxus::prelude::*;

use crate::{components::fill::FillBtn, models::domains::DomainInfo};

#[derive(Props)]
pub struct DomainCardProps<'a> {
    domain: &'a DomainInfo,
}

pub fn DomainCard<'a>(cx: Scope<'a, DomainCardProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "align-center",
            div {
                class: "w-full drop-shadow bg-neutral-700 text-gray-300 rounded m-1 p-2",
                div {
                    class: "grid grid-cols-2",
                    div {
                        "Username:"
                    },
                    div {
                        class: "grow",
                        "{cx.props.domain.username}"
                    }
                },
                div {
                    class: "grid grid-cols-2",
                    div {
                        "Password:"
                    },
                    div {
                        class: "grow",
                        "***********"
                    }
                }
                div {
                    class: "mt-4",
                    FillBtn {
                        domain: &cx.props.domain
                    }
                }
            }
        }
    })
}
