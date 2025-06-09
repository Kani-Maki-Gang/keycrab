use crate::{title::Title, domain::Domains};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-6">
            <Title />
            <Domains />
        </div>
    }
}
