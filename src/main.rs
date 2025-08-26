#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[cfg(not(feature = "server"))]
use ani_tracker::entry_web;

#[cfg(feature = "server")]
use ani_tracker::entry_server;


fn main() -> anyhow::Result<()> {
    #[cfg(not(feature = "server"))]
    return entry_web::run();

    #[cfg(feature = "server")]
    entry_server::run()
}

