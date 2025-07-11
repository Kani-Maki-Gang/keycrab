use keycrab_models::responses::DomainInfo;
use leptos::{prelude::*, task::spawn_local};

use super::api;
use crate::{browser, common::icon::IconButton, context::SettingsContext};

async fn write_password_to_clipboard(domain: DomainInfo, settings: Option<SettingsContext>) {
    let password = api::decrypt(domain.domain, domain.username, settings).await;
    if !password.is_empty() {
        let _ = browser::clipboard::write_text(password).await;
    }
}

#[component]
pub fn ClipboardButton(#[prop(into)] domain: Signal<DomainInfo>) -> impl IntoView {
    let settings = use_context::<RwSignal<SettingsContext>>();
    view! {
        <IconButton
            icon="iconoir-copy"
            on:click=move |_| {
                spawn_local(write_password_to_clipboard(domain.get(), settings.get()));
            }
        />
    }
}
