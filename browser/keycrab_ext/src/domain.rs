use crate::button::Button;
use leptos::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct DomainSearch {
    credentials: Vec<DomainInfo>
}

#[derive(Clone, Serialize, Deserialize)]
struct DomainInfo {
    pub id: i32,
    pub domain: String,
    pub username: String,
    pub password: String,
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
    let domains = LocalResource::new(async move || {
        let result = reqwest::get("http://localhost:3333/domain/search?q=%")
            .await;

        let Ok(result) = result else {
            return vec![];
        };

        let Ok(data) = result.json::<DomainSearch>().await else {
            return vec![];
        };

        data.credentials
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
