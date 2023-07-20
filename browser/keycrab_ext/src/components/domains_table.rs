use dioxus::prelude::*;

use crate::{models::domains::DomainInfo, components::fill::FillBtn};

#[derive(Props)]
pub struct DomainsTableProps<'a> {
    domains: &'a [DomainInfo],
}

pub fn DomainsTable<'a>(cx: Scope<'a, DomainsTableProps>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "p-4 flex flex-col grow",
            table {
                class: "table-auto border-collapse text-stone-900",
                thead {
                    class: "bg-slate-600",
                    tr {
                        th {
                            div {
                                class: "w-40 truncate text-left",
                                "Domain"
                            }
                        }
                        th {
                            div {
                                class: "w-40 truncate text-left",
                                "Username"
                            }
                        }
                        th {}
                    }
                }
                tbody {
                    class: "bg-slate-500",
                    for entry in cx.props.domains.iter() {
                        tr {
                            td {
                                div {
                                    class: "w-40 truncate",
                                    "{entry.url}"
                                }
                            }
                            td {
                                div {
                                    class: "w-40 truncate",
                                    "{entry.username}"
                                }
                            }
                            td {
                                FillBtn {}
                            }
                        }
                    }
                }
            }
        }
    })
}
