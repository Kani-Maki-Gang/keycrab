use keycrab_models::{
    requests::{DecryptQuery, SearchQuery},
    responses::{DomainInfo, DomainSearchResult},
};
use leptos::prelude::RwSignal;
use reqwest::Client;

pub async fn search(query: String) -> Vec<RwSignal<DomainInfo>> {
    let Ok(builder) = Client::builder().build() else {
        return vec![];
    };

    let query = SearchQuery {
        q: format!("%{query}%"),
    };

    let response = builder
        .get("http://localhost:3333/domain/search")
        .query(&query)
        .send()
        .await;

    let Ok(response) = response else {
        return vec![];
    };

    let Ok(data) = response.json::<DomainSearchResult>().await else {
        return vec![];
    };

    data.credentials.into_iter().map(RwSignal::new).collect()
}

pub async fn decrypt(domain: String, username: String) -> String {
    let Ok(builder) = Client::builder().build() else {
        return String::new();
    };

    let query = DecryptQuery { domain, username };

    let response = builder
        .get("http://localhost:3333/domain/decrypt")
        .query(&query)
        .send()
        .await;

    let Ok(response) = response else {
        return String::new();
    };

    response.json::<String>().await.unwrap_or_default()
}
