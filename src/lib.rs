pub mod backend;
pub mod configuration;

#[cfg(not(feature = "server"))]
pub mod entry_web;

#[cfg(feature = "server")]
pub mod entry_server;
mod client;