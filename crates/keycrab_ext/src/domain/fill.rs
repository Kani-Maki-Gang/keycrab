use anyhow::{Result, bail};
use keycrab_models::responses::DomainInfo;
use leptos::{prelude::*, task::spawn_local};

use crate::{
    browser::tab::{self, send_fill_args, send_fill_command},
    button::IconButton,
};

use super::api;

async fn fill_credentials(domain: DomainInfo) -> Result<()> {
    let active_tab = tab::query_active().await?;

    let Some(active_tab_id) = active_tab.id.as_ref() else {
        bail!("active tab doesn't have an id");
    };

    let password = api::decrypt(domain.domain, domain.username.clone()).await;

    if !password.is_empty() {
        let args = send_fill_args(&domain.username, &password)?;
        send_fill_command(*active_tab_id, args).await?;
    }

    Ok(())
}

async fn fill(domain: DomainInfo) {
    let _ = fill_credentials(domain);
}

#[component]
pub fn FillButton(#[prop(into)] domain: Signal<DomainInfo>) -> impl IntoView {
    view! {
        <IconButton
            icon="iconoir-input-field"
            on:click=move |_| {
                spawn_local(fill(domain.get()));
            }
        />
    }
}
