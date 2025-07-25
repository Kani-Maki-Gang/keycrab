mod api;
mod clipboard;
mod entry;
mod fill;

use crate::{
    context::{SearchContext, SettingsContext},
    title::Title,
};
use entry::DomainEntry;
use leptos::prelude::*;
use leptos_use::signal_debounced;

#[component]
pub fn Domains() -> impl IntoView {
    let search = use_context::<RwSignal<SearchContext>>();
    let settings = use_context::<RwSignal<SettingsContext>>();
    let query = Signal::derive(move || search.map(|x| x.get().query).unwrap_or_default());
    let query_debounced = signal_debounced(query, 500.0);
    let missing_settings = move || {
        let settings = settings.get();
        settings
            .map(|x| x.host.is_empty() || x.port.is_empty())
            .unwrap_or_default()
    };
    let domains = LocalResource::new(move || api::search(query_debounced.get(), settings.get()));
    view! {
        <div class="fixed top-0 right-0 left-0 flex flex-col border-b-1 border-slate-600 z-40">
            <Title />
        </div>
        <Show when=move || missing_settings() fallback=|| {}>
            <div class="fixed top-32 right-0 left-0 grid place-items-center">
                <div class="flex items-center gap-2 border border-yellow-300 rounded px-2 py-1 w-72">
                    <i class="iconoir-warning-triangle text-yellow-300 text-xl"></i>
                    "You need to update your settings"
                </div>
            </div>
        </Show>
        <div class="flex flex-col divide-y divide-slate-600 mt-24">
            <Show
                when=move || domains.get().is_some()
                fallback=move || view! { <div class="text-xl text-gray-400">"Loading..."</div> }
            >
                <For each=move || domains.get().unwrap() key=|x| x.get().id let:child>
                    <DomainEntry domain=child />
                </For>
            </Show>
        </div>
    }
}
