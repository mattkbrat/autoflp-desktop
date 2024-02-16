#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

// dx serve --hot-reload --platform desktop

use dioxus::html::geometry::euclid::num::Floor;
use dioxus::prelude::*;
use dioxus_desktop::{LogicalSize, WindowBuilder};
use dioxus_router::prelude::*;
mod lib;
mod tests;
mod client;

// ANCHOR_END: router

fn main() {

    let window = WindowBuilder::new()
        .with_title("AutoFLP")
        .with_resizable(true)
        .with_inner_size(LogicalSize::new(1000.0, 800.0))
        .with_min_inner_size(LogicalSize::new(700.0, 700.0));

    dioxus_desktop::launch_cfg(
        client::App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="dist/styles.css">"#.to_string())
            .with_window(window)
        ,
    )
}
