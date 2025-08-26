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
            div {
                class: "ani-info-image-container",
                a {
                    href: props.detail_url.clone(),
                    target: "_blank",

                    AniImage {
                        image_url: props.image_url.clone(),
                        title: props.title.clone(),
                    }
                }
            }
            // 动漫信息部分
            div { class: "ani-info-text-container",
                h3 {
                    class: "ani-info-title",
                    { props.title }
                }
                div {
                    class: "ani-info-update-time",
                    {props.update_time} "更新"
                }
                div {
                    class: "ani-info-update-count",
                    if !props.update_count.is_empty() {
                        "更新至第" {props.update_count} "集",
                    } else {
                        "暂无更新信息"
                    }
                }
                div {
                    class: "ani-info-platform",
                    span {
                        class: "ani-info-platform-tag",
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