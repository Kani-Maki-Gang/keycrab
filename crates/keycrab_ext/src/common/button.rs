use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button
          class="px-12 inline-block rounded-sm border border-indigo-600 bg-indigo-600 font-medium text-white focus:ring-3 focus:outline-hidden hover:cursor-pointer"
        >
            {children()}
        </button>
    }
}
