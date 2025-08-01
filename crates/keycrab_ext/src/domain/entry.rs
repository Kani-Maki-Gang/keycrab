use keycrab_models::responses::DomainInfo;
use leptos::prelude::*;

use crate::{
    common::icon::IconButton,
    context::SettingsContext,
    domain::{api, clipboard::ClipboardButton, fill::FillButton},
};

async fn get_password(domain: DomainInfo, show: bool, settings: Option<SettingsContext>) -> String {
    if show {
        api::decrypt(domain.domain, domain.username, settings).await
    } else {
        String::new()
    }
}

#[component]
pub fn DomainEntry(#[prop(into)] domain: Signal<DomainInfo>) -> impl IntoView {
    let settings = use_context::<RwSignal<SettingsContext>>();
    let show_password = RwSignal::new(false);
    let password =
        LocalResource::new(move || get_password(domain.get(), show_password.get(), settings.get()));
    let icon = Signal::derive(move || {
        if show_password.get() {
            "iconoir-eye-closed"
        } else {
            "iconoir-eye-solid"
        }
        .to_string()
    });
    view! {
        <div class="flex items-center gap-4 py-4 px-6 hover:bg-gray-900/10">
            <div class="size-7 rounded-md bg-red-900 text-center text-2xl grid place-items-center">
                <i class="iconoir-lock-square"></i>
            </div>
            <div class="flex flex-col grow">
                <span class="text-lg">{move || domain.get().domain}</span>
                <div class="flex gap-2 text-md">
                    <span class="font-semibold">username:</span>
                    {move || domain.get().username}
                </div>
                <div class="flex gap-2 text-md">
                    <span class="font-semibold">password:</span>
                    <Show
                        when=move || show_password.get()
                        fallback=move || view! { "•••••••••••••" }
                    >
                        {move || password.get()}
                    </Show>
                </div>
            </div>
            <FillButton domain=domain />
            <ClipboardButton domain=domain />
            <IconButton
                icon=icon
                on:click=move |_| {
                    show_password.update(|x| *x = !(*x));
                }
            />
        </div>
    }
}
