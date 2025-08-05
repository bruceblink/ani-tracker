use dioxus::prelude::*;
use crate::components::ani_item::AniItem;
pub(crate) use crate::components::AniData;

#[component]
pub fn AniList(items: Vec<AniData>) -> Element {
    
    rsx! {
        div { 
            class: "ani-list",
            style: " display: flex;
                     flexWrap: wrap;
                     gap: min(24px, 2vw);
                     justifyContent: center;
                     padding: 80px 0 24px;
                     margin: 0 auto;
                     width: 100%;
                     maxWidth: 90%;",
            
                // 使用 for 循环渲染列表项
                for data in items.iter() {
                    div { 
                        key:   {data.title},
                        style: "width: calc(clamp(360px, calc(90vw/4 - 24px), 360px));
                                height: calc(calc(clamp(360px, calc(90vw/4 - 24px), 360px) * 0.618));
                                flexShrink: 0;",
                        AniItem {
                            title: {data.title.clone()},
                            update_count: {data.update_count.clone()},
                            detail_url:   {data.detail_url.clone()},
                            update_info:  {data.update_info.clone()},
                            image_url:    {data.image_url.clone()},
                            update_time:  {data.update_time.clone()},
                            platform:     {data.platform.clone()},
                        }
                    }
            }
        }
    }
}
