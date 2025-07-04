use crate::{
    common::{button::Button, link::IconLink, checkbox::Checkbox, field::TextField},
    context::SettingsContext,
};
use leptos::prelude::*;

#[component]
pub fn Settings() -> impl IntoView {
    let settings = use_context::<RwSignal<SettingsContext>>();
    let host = RwSignal::new(settings.map(|x| x.get_untracked().host).unwrap_or_default());
    let port = RwSignal::new(settings.map(|x| x.get_untracked().host).unwrap_or_default());
    let tls = RwSignal::new(settings.map(|x| x.get_untracked().tls).unwrap_or(true));

    view! {
        <div class="flex items-center gap-4 px-6 h-24 bg-gray-800 text-3xl">
            <span class="grow">"Settings"</span>
            <IconLink icon="iconoir-xmark" href="/" />
        </div>
        <div class="flex flex-col gap-4 py-4 mx-6">
            <TextField id="host" label="host" value=host />
            <TextField id="port" label="port" value=port />
            <Checkbox id="tls" label="tls" value=tls />
            <div class="flex justify-end">
                <Button>"Save"</Button>
            </div>
        </div>
    }
}
