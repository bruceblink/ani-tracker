use dioxus::prelude::*;
use dioxus::prelude::server_fn::serde::Deserialize;
use serde::Serialize;
use crate::components::ani_item::AniItem;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AniData {
    pub title: String,
    pub url:   String,
}

#[component]
pub fn AniList(items: Vec<AniData>) -> Element {
    
    rsx! {
        div { id: "ani-list",
            div { id: "links",
                // 使用 for 循环渲染列表项
                for data in items.iter() {
                    AniItem {
                        key:   "{data.title}",
                        title: "{data.title}",
                        url:   "{data.url}",
                    }
                }
            }
        }
    }
}
