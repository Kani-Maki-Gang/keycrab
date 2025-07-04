mod app;
mod browser;
mod common;
mod context;
mod domain;
mod models;
mod search;
mod settings;
mod title;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
