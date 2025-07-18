use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(into)] id: Signal<String>,
    #[prop(into)] label: Signal<String>,
    #[prop(into)] value: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-32">
            <label id=move || id.get() class="text-lg">
                {move || label.get()}
            </label>
            <div class="grow pl-4">
                <input
                    id=move || id.get()
                    type="checkbox"
                    class="size-5 rounded border-gray-300 shadow-sm bg-gray-800 border-gray-600 ring-offset-gray-900 checked:bg-blue-600"
                    prop:checked=move || value.get()
                    on:change=move |ev| value.set(event_target_checked(&ev))
                />
            </div>
        </div>
    }
}
