use dioxus::prelude::*;
use crate::client::components::ani_item::AniItem;
pub(crate) use crate::client::components::AniData;
const ANILIST_CSS: Asset = asset!("/assets/styling/aniList.css");
#[component]
pub fn AniList(items: Vec<AniData>) -> Element {
    
    rsx! {
        document::Link { rel: "stylesheet", href: ANILIST_CSS }
        div { 
            class: "ani-list",
                // 使用 for 循环渲染列表项
            for data in items.iter() {
                div {
                    key:   {data.title.clone()},
                    style: "width: calc(clamp(360px, calc(90vw/4 - 24px), 360px));
                            height: calc(calc(clamp(360px, calc(90vw/4 - 24px), 360px) * 0.618));
                            flex-shrink: 0;",
                    AniItem {
                        title: data.title.clone(),
                        update_count: data.update_count.clone(),
                        detail_url:   data.detail_url.clone(),
                        update_info:  data.update_info.clone(),
                        image_url:    data.image_url.clone(),
                        update_time:  data.update_time.clone(),
                        platform:     data.platform.clone(),
                    }
                }
            }
        }
    }
}
