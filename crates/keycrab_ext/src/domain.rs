use crate::{button::Button, context::SearchContext};
use keycrab_models::responses::{DomainInfo, DomainSearchResult};
use leptos::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
struct DomainQuery {
    q: String,
}

async fn get_domains(query: String) -> Vec<DomainInfo> {
    let Ok(builder) = Client::builder().build() else {
        return vec![];
    };

    let query = DomainQuery { q: format!("%{query}%")};

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

    data.credentials
}

#[component]
fn Domain(#[prop(into)] domain_info: DomainInfo) -> impl IntoView {
    view! {
        <div class="hover:bg-slate-700 focus:rounded-xl focus:border-1 focus:border-slate-600">
            <div class="flex items-center gap-4 py-4 mx-6">
                <div class="size-7 rounded-md bg-red-900 text-center text-2xl grid place-items-center">
                    <i class="iconoir-lock-square"></i>
                </div>
                <div class="flex flex-col grow">
                    <span class="text-lg">{{ domain_info.domain }}</span>
                    <div class="flex gap-2 text-md">
                        <span class="font-semibold">username:</span>
                        {{ domain_info.username }}
                    </div>
                </div>
                <Button>"Fill"</Button>
            </div>
        </div>
    }
}

#[component]
pub fn Domains() -> impl IntoView {
    let search_context = use_context::<RwSignal<SearchContext>>();

    let domains = LocalResource::new(move || {
        get_domains(search_context.map(|x| x.get().query).unwrap_or_default())
    });

    view! {
        <div class="flex flex-col divide-y divide-slate-600">
            <Show
                when=move || domains.get().is_some()
                fallback=move || view! { <div class="text-xl text-gray-400">"Loading..."</div> }
            >
                <For each=move || domains.get().unwrap() key=|x| x.id let:child>
                    <Domain domain_info=child />
                </For>
            </Show>
        </div>
    }
}
