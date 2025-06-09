mod app;
mod button;
mod domain;
mod title;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
