use anyhow::{anyhow, bail, Result};
use dioxus::prelude::*;
use reqwest::Client;

use crate::{
    browser::tab,
    components::{
        domain_card::DomainCard, domains_empty::DomainsEmpty, domains_error::DomainsError,
        domains_loading::DomainsLoading,
    },
    models::domains::{DomainInfo, SearchResponse},
};

async fn domain_search(url: &str) -> Result<SearchResponse> {
    Client::new()
        .get("http://localhost:8000/domain/search")
        .query(&[("q", url)])
        .send()
        .await?
        .json()
        .await
        .map_err(|e| anyhow!(e))
}

async fn get_domains() -> Result<Vec<DomainInfo>> {
    let active_tab = tab::query_active().await?;

    let Some(active_tab_id) = active_tab.id.as_ref() else {
        bail!("active tab doesn't have an id");
    };

    let Some(active_tab_url) = active_tab.url.as_ref() else {
        bail!("active tab doesn't contain a url");
    };

    let response = domain_search(active_tab_url).await?;

    let domains = response
        .credentials
        .into_iter()
        .map(|e| DomainInfo {
            tab_id: *active_tab_id,
            url: e.domain,
            username: e.username,
            password: e.password,
        })
        .collect();

    Ok(domains)
}

pub fn Domains(cx: Scope) -> Element {
    let load_fut = use_future(cx, (), |_| async move { get_domains().await });
    cx.render(match load_fut.value() {
        Some(Ok(domains)) if domains.len() > 0 => rsx! {
            div {
                class: "w-full flex flex-col p-4",
                for entry in domains.iter() {
                    DomainCard {
                        domain: entry
                    }
                }
            }
        },
        Some(Ok(_)) => rsx! {
            DomainsEmpty {}
        },
        Some(Err(e)) => rsx! {
            DomainsError {
                error: &e
            }
        },
        None => rsx! {
            DomainsLoading {}
        },
    })
}
