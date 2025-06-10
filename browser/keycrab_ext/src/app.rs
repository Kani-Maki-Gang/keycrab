use crate::{title::Title, domain::Domains};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-6">
            <div class="fixed top-0 right-0 left-0 border-b-1 border-slate-600 px-6 z-40">
                <Title />
            </div>
            <div class="pt-18">
                <Domains />
            </div>
        </div>
    }
}
