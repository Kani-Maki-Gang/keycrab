mod app;
mod browser;
mod button;
mod context;
mod domain;
mod models;
mod search;
mod title;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
