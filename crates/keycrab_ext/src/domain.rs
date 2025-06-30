use crate::{
    button::IconButton,
    context::SearchContext,
};
use js_sys::Promise;
use keycrab_models::responses::{DomainInfo, DomainSearchResult};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_use::signal_debounced;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["navigator", "clipboard"])]
    fn writeText(value: String) -> Promise;
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct DomainQuery {
    q: String,
}

async fn get_domains(query: String) -> Vec<RwSignal<DomainInfo>> {
    let Ok(builder) = Client::builder().build() else {
        return vec![];
    };

    let query = DomainQuery {
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

fn write_password_to_clipboard(password: String) {
    let _ = writeText(password);
}

#[component]
fn Domain(#[prop(into)] domain_info: Signal<DomainInfo>) -> impl IntoView {
    let show_password = RwSignal::new(false);
    let password = move || {
        if show_password.get() {
            domain_info.get().password
        } else {
            "•••••••••••••".to_string()
        }
    };
    let icon = Signal::derive(move || {
        if show_password.get() {
            "iconoir-eye-closed"
        } else {
            "iconoir-eye-solid"
        }
        .to_string()
    });
    view! {
        <div class="hover:bg-slate-700 focus:rounded-xl focus:border-1 focus:border-slate-600">
            <div class="flex items-center gap-4 py-4 mx-6">
                <div class="size-7 rounded-md bg-red-900 text-center text-2xl grid place-items-center">
                    <i class="iconoir-lock-square"></i>
                </div>
                <div class="flex flex-col grow">
                    <span class="text-lg">{move || domain_info.get().domain}</span>
                    <div class="flex gap-2 text-md">
                        <span class="font-semibold">username:</span>
                        {move || domain_info.get().username}
                    </div>
                    <div class="flex gap-2 text-md">
                        <span class="font-semibold">password:</span>
                        {move || password()}
                    </div>
                </div>
                <IconButton icon="iconoir-input-field" />
                <IconButton
                    icon="iconoir-copy"
                    on:click=move |_| write_password_to_clipboard(domain_info.get().password)
                />
                <IconButton icon=icon on:click=move |_| show_password.update(|x| *x = !(*x)) />
            </div>
        </div>
    }
}

#[component]
pub fn Domains() -> impl IntoView {
    let search_context = use_context::<RwSignal<SearchContext>>();
    let query = Signal::derive(move || search_context.map(|x| x.get().query).unwrap_or_default());
    let query_debounced = signal_debounced(query, 500.0);

    let domains = LocalResource::new(move || get_domains(query_debounced.get()));

    view! {
        <div class="flex flex-col divide-y divide-slate-600">
            <Show
                when=move || domains.get().is_some()
                fallback=move || view! { <div class="text-xl text-gray-400">"Loading..."</div> }
            >
                <For each=move || domains.get().unwrap() key=|x| x.get().id let:child>
                    <Domain domain_info=child />
                </For>
            </Show>
        </div>
    }
}
