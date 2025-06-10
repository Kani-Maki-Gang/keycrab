use crate::button::IconButton;
use leptos::prelude::*;

#[component]
pub fn Title() -> impl IntoView {
    view! {
        <div class="flex flex-row items-center gap-4 h-24 w-full bg-slate-800">
            <img class="size-14 z-40" src="./icons/keycrab-logo-ext.png" />
            <div class="grow text-4xl">
                <span class="text-gray-500">"Key"</span>
                <span class="text-white">"crab"</span>
            </div>
            <IconButton icon="iconoir-settings" />
        </div>
    }
}
