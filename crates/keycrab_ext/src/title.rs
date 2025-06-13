use crate::{button::IconButton, search::Search};
use leptos::prelude::*;

#[component]
pub fn Title() -> impl IntoView {
    view! {
        <div class="grid items-center h-24 bg-gray-800">
            <div class="flex flex-row items-center gap-4 px-6">
                <img class="size-12 z-40" src="./icons/keycrab-logo-ext.png" />
                <div class="grow text-3xl">
                    <span class="text-gray-500">"Key"</span>
                    <span class="text-white">"crab"</span>
                </div>
                <div class="max-w-64 w-full">
                    <Search />
                </div>
                <IconButton icon="iconoir-settings" />
            </div>
        </div>
    }
}
