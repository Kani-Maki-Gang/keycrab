use crate::{
    context::{SearchContext, SettingsContext},
    domain::Domains,
    settings::Settings,
};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{components::*, path};
use leptos_use::storage::use_local_storage;

#[component]
pub fn App() -> impl IntoView {
    let (storage, _, _) = use_local_storage::<SettingsContext, JsonSerdeCodec>("keycrabSettings");
    let settings = RwSignal::new(storage.get_untracked());
    let search = RwSignal::new(SearchContext::default());
    provide_context(search);
    provide_context(settings);

    view! {
        <Router>
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=Domains />
                <Route path=path!("/settings") view=Settings />
                <Route path=path!("/*any") view=Domains />
            </Routes>
        </Router>
    }
}
