use anyhow::{Result, bail};
use keycrab_models::responses::DomainInfo;
use leptos::{leptos_dom::logging::console_error, prelude::*, task::spawn_local};

use crate::{
    browser::{
        script::load_fill_form,
        tab::{self, send_fill_command},
    },
    common::icon::IconButton, context::SettingsContext,
};

use super::api;

async fn fill_credentials(domain: DomainInfo, settings: Option<SettingsContext>) -> Result<()> {
    let active_tab = tab::get_current().await?;

    let Some(active_tab_id) = active_tab.id.as_ref() else {
        bail!("active tab doesn't have an id");
    };

    load_fill_form(active_tab_id).await?;

    let password = api::decrypt(domain.domain, domain.username.clone(), settings).await;

    if !password.is_empty() {
        send_fill_command(*active_tab_id, &domain.username, &password).await?;
    }

    Ok(())
}

async fn fill(domain: DomainInfo, settings: Option<SettingsContext>) {
    if let Err(e) = fill_credentials(domain, settings).await {
        console_error(&e.to_string());
    }
}

#[component]
pub fn FillButton(#[prop(into)] domain: Signal<DomainInfo>) -> impl IntoView {
    let settings = use_context::<RwSignal<SettingsContext>>();
    view! {
        <IconButton
            icon="iconoir-input-field"
            on:click=move |_| {
                spawn_local(fill(domain.get(), settings.get()));
            }
        />
    }
}
