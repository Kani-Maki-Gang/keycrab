use leptos::prelude::*;

#[component]
pub fn TextInput() -> impl IntoView {
    view! {
        <input
            type="text"
            class="p-2 h-[36px] w-full rounded border-1 border-slate-600 bg-gray-800 text-white"
        />
    }
}
