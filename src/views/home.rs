use dioxus::prelude::*;
use crate::components::ani_list::{AniData, AniList};
use crate::components::search::search_server;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let query = consume_context::<Signal<String>>();
    let results = use_signal(|| Vec::<AniData>::new());

    use_effect(move || {
        let current_query = query(); // 获取最新搜索词
        let mut results = results.clone();

        spawn(async move {
            let data = search_server(current_query.clone()).await.unwrap_or_default();
            results.set(data);
        });
    });
    rsx! {
        
        div {
            class: "home",
            style: "padding-top: var(--navbar-height);",
            if results().is_empty() {
                p {
                    class: "no-results",
                    "No results found."
                }
            } else {
                AniList { items: results() }
            }
        }
    }
}



