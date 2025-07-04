use crate::{
    context::{SearchContext, SettingsContext},
    domain::Domains,
    settings::Settings,
};
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    let search = RwSignal::new(SearchContext::default());
    let settings = RwSignal::new(SettingsContext::default());
    provide_context(search);
    provide_context(settings);

    view! {
        <Router>
            <Routes fallback=|| "Not found">
                <Route path=path!("") view=Domains />
                <Route path=path!("/settings") view=Settings />
            </Routes>
        </Router>
    }
}
