use crate::{
    common::{button::Button, checkbox::Checkbox, field::TextField, link::IconLink},
    context::SettingsContext,
};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{NavigateOptions, hooks::use_navigate};
use leptos_use::storage::use_local_storage;

#[component]
pub fn Settings() -> impl IntoView {
    let nav = use_navigate();
    let (_, set_storage, _) =
        use_local_storage::<SettingsContext, JsonSerdeCodec>("keycrabSettings");
    let settings = use_context::<RwSignal<SettingsContext>>();
    let host = RwSignal::new(settings.map(|x| x.get_untracked().host).unwrap_or_default());
    let port = RwSignal::new(settings.map(|x| x.get_untracked().port).unwrap_or_default());
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
                <Button on:click=move |_| {
                    let new_settings = SettingsContext::new(host.get(), port.get(), tls.get());
                    set_storage.set(new_settings.clone());
                    if let Some(settings) = settings {
                        settings.set(new_settings);
                    }
                    nav("/", NavigateOptions::default());
                }>"Save"</Button>
            </div>
        </div>
    }
}
