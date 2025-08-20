#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;
mod utils;
mod route;
mod backend;
// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.



const DEV_URL: &str = "http://localhost:8080";
const PROD_URL: &str = "https://ani-tracker.fly.dev";

fn setup_server_api_url() {
    let url = if cfg!(debug_assertions) { DEV_URL } else { PROD_URL };
    server_fn::client::set_server_url(url);
}

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    #[cfg(not(feature = "server"))]
    setup_server_api_url();
    launch(views::App);
}

