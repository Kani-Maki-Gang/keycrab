use leptos::prelude::*;

#[component]
pub fn IconButton(#[prop(into)] icon: Signal<String>) -> impl IntoView {
    view! {
        <button class="grid place-items-center rounded-full border border-indigo-600 bg-indigo-600 size-10 p-2 text-xl font-medium text-white focus:ring-3 focus:outline-hidden hover:cursor-pointer">
            <i class=move || icon.get()></i>
        </button>
    }
}
