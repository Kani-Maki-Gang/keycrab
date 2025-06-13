mod app;
mod button;
mod context;
mod domain;
mod search;
mod title;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
