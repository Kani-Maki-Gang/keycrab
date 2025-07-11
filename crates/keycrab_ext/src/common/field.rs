use super::input::TextInput;
use leptos::prelude::*;

#[component]
pub fn TextField(
    #[prop(into)] id: Signal<String>,
    #[prop(into)] label: Signal<String>,
    #[prop(into)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex gap-32">
            <label for=move || id.get() class="text-lg">
                {move || label.get()}
            </label>
            <div class="grow ">
                <TextInput
                    attr:id=move || id.get()
                    prop:value=move || value.get()
                    on:input=move |ev| value.set(event_target_value(&ev))
                />
            </div>
        </div>
    }
}
