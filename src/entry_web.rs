use dioxus::prelude::*;


fn setup_server_api_url() {
    const DEV_URL: &str = "http://localhost:8080";
    const PROD_URL: &str = "https://ani-tracker.fly.dev";
    let url = if cfg!(debug_assertions) { DEV_URL } else { PROD_URL };
    server_fn::client::set_server_url(url);
}
#[cfg(not(feature = "server"))]
pub fn run() -> anyhow::Result<()> {
    // wasm/web 模式下设置 API 地址
    setup_server_api_url();

    // 启动前端 Dioxus app
    launch(crate::views::App);

    Ok(())
}
