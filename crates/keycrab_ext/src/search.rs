use crate::{common::input::TextInput, context::SearchContext};
use leptos::prelude::*;

#[component]
pub fn Search() -> impl IntoView {
    let search_context = use_context::<RwSignal<SearchContext>>();

    view! {
        <Show when=move || search_context.is_some() fallback=move || view! {}>
            <TextInput
                attr:id="search"
                attr:placeholder="Search"
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
