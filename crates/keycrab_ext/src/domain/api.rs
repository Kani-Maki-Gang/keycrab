use keycrab_models::{
    requests::{DecryptQuery, SearchQuery},
    responses::{DomainInfo, DomainSearchResult},
};
use leptos::prelude::RwSignal;
use reqwest::Client;

use crate::context::SettingsContext;

pub async fn search(query: String, settings: Option<SettingsContext>) -> Vec<RwSignal<DomainInfo>> {
    let Some(settings) = settings else {
        return vec![];
    };

    let Ok(builder) = Client::builder().build() else {
        return vec![];
    };

    let query = SearchQuery {
        q: format!("%{query}%"),
    };

    let url = format!("{}/domain/search", settings.base_url());
    let response = builder.get(url).query(&query).send().await;

    let Ok(response) = response else {
        return vec![];
    };

    let Ok(data) = response.json::<DomainSearchResult>().await else {
        return vec![];
    };

    data.credentials.into_iter().map(RwSignal::new).collect()
}

pub async fn decrypt(
    domain: String,
    username: String,
    settings: Option<SettingsContext>,
) -> String {
    let Some(settings) = settings else {
        return String::new();
    };

    let Ok(builder) = Client::builder().build() else {
        return String::new();
    };

    let query = DecryptQuery { domain, username };

    let url = format!("{}/domain/decrypt", settings.base_url());
    let Ok(response) = builder.get(url).query(&query).send().await else {
        return String::new();
    };
    response.json::<String>().await.unwrap_or_default()
}
