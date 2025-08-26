mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
pub mod views;
mod utils;
mod route;
pub mod backend;
mod configuration;

#[cfg(not(feature = "server"))]
pub mod entry_web;

#[cfg(feature = "server")]
pub mod entry_server;