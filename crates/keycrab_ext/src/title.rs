use crate::{common::link::IconLink, context::SettingsContext, search::Search};
use leptos::prelude::*;

#[component]
pub fn Title() -> impl IntoView {
    let settings = use_context::<RwSignal<SettingsContext>>();
    let missing_settings = move || {
        let settings = settings.get();
        settings
            .map(|x| x.host.is_empty() || x.port.is_empty())
            .unwrap_or_default()
    };
    view! {
        <div class="grid items-center h-24 bg-gray-800">
            <div class="flex flex-row items-center gap-4 px-6">
                <img class="size-12 z-40" src="./icons/keycrab-logo-ext.png" />
                <div class="grow text-3xl">
                    <span class="text-gray-500">"Key"</span>
                    <span class="text-white">"crab"</span>
                </div>
                <Show when=move || missing_settings() fallback=|| {}>
                    <div class="flex items-center gap-2 border border-yellow-300 rounded px-2 py-1">
                        <i class="iconoir-warning-triangle text-yellow-300 text-xl"></i>
                        "You need to update your settings"
                    </div>
                </Show>
                <div class="max-w-64 w-full">
                    <Search />
                </div>
                <IconLink icon="iconoir-settings" href="/settings" />
            </div>
        </div>
    }
}
