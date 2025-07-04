use leptos::prelude::*;
use crate::common::{button::Button, field::TextField, checkbox::Checkbox};

#[component]
pub fn Settings() -> impl IntoView {
    let host = RwSignal::new(String::new());
    let port = RwSignal::new(String::new());
    let tls = RwSignal::new(true);
    view! {
        <div class="flex flex-col gap-4 py-4 mx-6">
            <TextField id="host" label="host" value=host />
            <TextField id="port" label="port" value=port />
            <Checkbox id="tls" label="tls" value=tls />
            <div class="flex justify-end gap-4">
                <Button>"Save"</Button>
                <Button>"Cancel"</Button>
            </div>
        </div>
    }
}
