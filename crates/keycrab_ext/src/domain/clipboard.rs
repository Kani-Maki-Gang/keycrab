use keycrab_models::responses::DomainInfo;
use leptos::{prelude::*, task::spawn_local};

use super::api;
use crate::{browser, common::icon::IconButton};

async fn write_password_to_clipboard(domain: DomainInfo) {
    let password = api::decrypt(domain.domain, domain.username).await;
    if !password.is_empty() {
        let _ = browser::clipboard::write_text(password).await;
    }
}

#[component]
pub fn ClipboardButton(#[prop(into)] domain: Signal<DomainInfo>) -> impl IntoView {
    view! {
        <IconButton
            icon="iconoir-copy"
            on:click=move |_| {
                spawn_local(write_password_to_clipboard(domain.get()));
            }
        />
    }
}
