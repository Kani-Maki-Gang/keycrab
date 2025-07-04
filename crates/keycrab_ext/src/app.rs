use crate::{context::SearchContext, domain::Domains, title::Title, settings::Settings};
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    let search = RwSignal::new(SearchContext::default());
    provide_context(search);

    view! {
        <Router>
            <div class="fixed top-0 right-0 left-0 border-b-1 border-slate-600 z-40">
                <Title />
            </div>
            <div class="flex flex-col gap-4 pt-24">
                <Routes fallback=|| "Not found">
                    <Route path=path!("") view=Domains />
                    <Route path=path!("/settings") view=Settings />
                </Routes>
            </div>
        </Router>
    }
}
