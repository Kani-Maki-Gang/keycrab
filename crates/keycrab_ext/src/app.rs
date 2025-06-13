use crate::{domain::Domains, title::Title};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <div class="fixed top-0 right-0 left-0 border-b-1 border-slate-600 z-40">
                <Title />
            </div>
            <div class="pt-24">
                <Domains />
            </div>
        </div>
    }
}
