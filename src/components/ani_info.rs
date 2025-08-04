use dioxus::prelude::*;
use crate::components::ani_item::AniItem;
pub(crate) use crate::components::AniData;

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
