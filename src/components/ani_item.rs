use dioxus::prelude::*;

const ANIITEM_CSS: Asset = asset!("/assets/styling/aniItem.css");
#[derive(Props, PartialEq, Clone)]
pub struct AniProps {
    #[props(into)]
    title: String,
    #[props(into)]
    detail_url: String,
    #[props(into)]
    update_count: String,
    #[props(into)]
    update_info: String,
    #[props(into)]
    image_url: String,
    #[props(into)]
    update_time: String,
    #[props(into)]
    platform: String,
}

#[component]
pub fn AniItem(props: AniProps) -> Element {
    let info_props = props.clone();
    rsx! {
        document::Link { rel: "stylesheet", href: ANIITEM_CSS }
        div {
            class: "ani-item",
            // 直接把整个 props（按值）展开给 AniInfo
            AniInfo { ..info_props }
        }
    }
}


#[component]
pub fn AniInfo(props: AniProps) -> Element {

    rsx! {
        div {
            class: "ani-info",
            style: "display: flex;
                    flex-direction: row;
                    align-items: flex-start;
                    gap: 16px;
                    padding: 32px;
                    width: 100%;
                    min-height: 0;
                    overflow: hidden;
                    flex-shrink: 1;",
            div {
                style: "position: relative; flex-shrink: 0;",
                a {
                    href: props.detail_url.clone(),
                    target: "_blank",
                    style: "display: block;
                            width: 92.8px;
                            height: 150px;
                            border-radius: 8px;
                            overflow: hidden;
                            box-shadow: 0 2px 4px rgba(0,0,0,0.1);",

                    AniImage {
                        image_url: props.image_url.clone(),
                        title: props.title.clone(),
                    }
                }
            }
            // 动漫信息部分
            div {style: "display: flex;flex: 1;min-width: 0;flex-direction: column; align-items: center;text-align: center;",
                h3 {
                    style: "
                      margin: 0 0 8px;
                      font-size: 1.1rem;
                      font-weight: 600;
                      color: #333;
                      max-width: 100%;
                      word-wrap: break-word;
                      display: -webkit-box;
                      -webkit-line-clamp: 2;
                      -webkit-box-orient: vertical;
                      overflow: hidden;
                      line-height: 1.4;
                    ",
                    { props.title }
                }
                div {
                    style: "font-size: 0.9rem;
                            color: #666;
                            margin-bottom: 4px;
                            width: 100%;",
                    {props.update_time} "更新"
                }
                div {
                    style: "font-size: 0.9rem;
                            color: #666;
                            margin-bottom: 8px;
                            width: 100%;",
                    if !props.update_count.is_empty() {
                        "更新至第" {props.update_count} "集",
                    } else {
                        "暂无更新信息"
                    }
                }
                div {
                    style: "font-size: 0.85rem;
                            color: #888;
                            align-items: center;
                            gap: 8;",
                    span {
                        style: "padding: 2px 8px;
                                background: #f5f5f5;
                                border-radius: 4px;",
                        {props.platform}
                    }

                }
            }
        }
    }
}


#[component]
pub fn AniImage(image_url: String, title: String) -> Element {

    rsx! {
        img {
            src: "{image_url}",
            alt: "{title}",
            style: "width: 100%;
                    height: 100%;
                    object-fit: cover;"
        }
    }
}