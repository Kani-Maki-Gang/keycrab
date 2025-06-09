use crate::button::IconButton;
use leptos::prelude::*;

#[component]
pub fn Title() -> impl IntoView {
    view! {
        <div class="flex items-center gap-4">
            <img class="size-14" src="./icons/keycrab-logo-ext.png" />
            <div class="grow text-4xl">
                <span class="text-gray-500">"Key"</span>
                <span class="text-white">"crab"</span>
            </div>
            <IconButton icon="iconoir-settings" />
        </div>
    }
}
