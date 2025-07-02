use crate::{context::SearchContext, domain::Domains, title::Title};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let search = RwSignal::new(SearchContext::default());
    provide_context(search);

    view! {
        <div class="fixed top-0 right-0 left-0 border-b-1 border-slate-600 z-40">
            <Title />
        </div>
        <div class="flex flex-col gap-4 pt-24">
            <Domains />
        </div>
    }
}
