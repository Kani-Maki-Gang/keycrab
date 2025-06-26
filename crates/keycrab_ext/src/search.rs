use leptos::prelude::*;

use crate::context::SearchContext;

#[component]
pub fn Search() -> impl IntoView {
    let search_context = use_context::<RwSignal<SearchContext>>();

    view! {
        <Show when=move || search_context.is_some() fallback=move || view! {}>
            <input
                type="text"
                id="search"
                class="p-2 h-[36px] w-full rounded border-1 border-slate-600 bg-gray-800 text-white"
                placeholder="Search"
                prop:value=move || search_context.map(|x| x.get().query).unwrap_or_default()
                on:input=move |ev| {
                    if let Some(search_context) = search_context {
                        search_context.update(|x| x.query = event_target_value(&ev));
                    }
                }
            />
        </Show>
    }
}
