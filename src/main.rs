#![allow(non_snake_case)]

// dx serve --hot-reload --platform desktop

use dioxus::html::geometry::euclid::num::Floor;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod lib;
mod tests;
mod client;

// ANCHOR_END: router


fn main() {
    dioxus_desktop::launch_cfg(
        client::App,
        dioxus_desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="dist/styles.css">"#.to_string()),
    )
}
