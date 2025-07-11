use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn IconLink(
    #[prop(into)] icon: Signal<String>,
    #[prop(into)] href: Signal<String>,
) -> impl IntoView {
    view! {
        <A
            attr:class="grid place-items-center rounded-full bg-gray-800 size-10 p-2 text-xl font-medium text-white focus:ring-3 focus:outline-hidden focus:bg-gray-900 hover:bg-gray-900 hover:cursor-pointer"
            href=move || href.get()
        >
            <i class=move || icon.get()></i>
        </A>
    }
}
