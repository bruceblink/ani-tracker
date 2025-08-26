use dioxus::prelude::*;
use crate::client::components::ani_list::{AniData, AniList};
use crate::client::components::search::search;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let query = consume_context::<Signal<String>>();
    let results = use_signal(|| Vec::<AniData>::new());
    let loading = use_signal(|| true); // 添加一个 loading 状态

    use_effect(move || {
        let current_query = query();
        let mut results = results.clone();
        let mut loading = loading.clone();

        spawn(async move {
            loading.set(true); // 开始加载，设置 loading 为 true
            let data = search(current_query.clone()).await.unwrap_or_default();
            results.set(data);
            loading.set(false); // 加载完成，设置 loading 为 false
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div {
            class: "home",
            if *loading.read() {
                // 在加载时显示一个占位符，例如加载动画或骨架屏
                p {
                    class: "loading",
                    "加载中..."
                }
            } else if results().is_empty() {
                p {
                    class: "no-results",
                    "没有找到结果。"
                }
            } else {
                AniList { items: results() }
            }
        }
    }
}



