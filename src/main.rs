use leptos::prelude::*;
mod app;
mod components;
mod version;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
