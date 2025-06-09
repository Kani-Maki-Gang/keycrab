use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="inline-block rounded-sm border border-indigo-600 bg-indigo-600 px-8 py-1 text-lg font-medium text-white focus:ring-3 focus:outline-hidden hover:cursor-pointer">
            {{ children() }}
        </button>
    }
}

#[component]
pub fn IconButton(#[prop(into)] icon: Signal<String>) -> impl IntoView {
    view! {
        <button class="grid place-items-center rounded-full border border-indigo-600 bg-indigo-600 size-12 p-2 text-2xl font-medium text-white focus:ring-3 focus:outline-hidden hover:cursor-pointer">
            <i class=move || icon.get()></i>
        </button>
    }
}
