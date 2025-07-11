use leptos::prelude::*;

#[component]
pub fn IconButton(#[prop(into)] icon: Signal<String>) -> impl IntoView {
    view! {
        <button class="grid place-items-center rounded-full bg-gray-800 size-10 p-2 text-xl font-medium text-white focus:ring-3 focus:outline-hidden focus:bg-gray-900 hover:bg-gray-900 hover:cursor-pointer">
            <i class=move || icon.get()></i>
        </button>
    }
}
