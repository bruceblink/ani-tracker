use dioxus::prelude::*;
use tokio::runtime::Runtime;
use crate::configuration::config::init_config;
use crate::backend::timer_tasker::start_async_timer_task;

pub fn run() -> anyhow::Result<()> {
    // 加载任务配置
    let task_config = init_config()?;

    // 手动创建 tokio runtime
    let rt = Runtime::new()?;

    // 异步启动定时任务
    rt.spawn(async move {
        start_async_timer_task(task_config).await;
    });

    // 启动 Dioxus 全栈 app（SSR/服务端渲染）
    launch(crate::views::App);

    // 阻塞主线程，保持 runtime 存活
    rt.block_on(async {
        tokio::signal::ctrl_c().await.unwrap();
    });

    Ok(())
}
