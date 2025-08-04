use crate::components::{Search, AniList};
use dioxus::prelude::*;
use crate::components::ani_info::AniData;
use crate::components::search::search_server;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut query = use_signal(|| String::new());
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
        link { rel: "stylesheet", href: asset!("/assets/styling/search.css") }

        div { id: "app",

            Search {
                on_search: move |new_q| query.set(new_q),
            }

            if results().is_empty() {
                p { "No results found." }
            } else {
                AniList { items: results() }
            }
        }
    }
}



