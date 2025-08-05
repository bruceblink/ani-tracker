use dioxus::prelude::*;

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
        div {
            class: "ani-item",
            id: format!("ani-item-{}", props.title),

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
            style: "border:1px solid #ccc;
                    padding:10px;
                    margin-bottom:10px;
                    display: flex;
                    flexDirection: row;
                    alignItems: flex-start;
                    gap: 16px;
                    padding: 32px;
                    width: 90%;
                    height: 216.3px;",
            div {
                style: "position: relative; flexShrink: 0;",
                a {
                    href: {props.detail_url.clone()},
                    target: "_blank",
                    style: "display: block;
                            width: 92.8px;
                            height: 150px;
                            borderRadius: 8px;
                            overflow: hidden;
                            boxShadow: 0 2px 4px rgba(0,0,0,0.1);",

                    AniImage {
                        image_url: props.image_url.clone(),
                        title: props.title.clone(),
                    }
                }
            }
            // 动漫信息部分
            div {
                style: "flex: 1; display: flex; flexDirection: column; justifyContent: space-between;",
                h3 {
                    style: "margin: 0 0 8px;
                            fontSize: 1.1rem;
                            fontWeight: 600;
                            color: #333;
                            maxWidth: 100%;
                            wordWrap: break-word;
                            display: -webkit-box;
                            WebkitLineClamp: 2;
                            WebkitBoxOrient: vertical;
                            overflow: hidden;
                            lineHeight: 1.4;",
                    {props.title}
                }
                div {
                    style: "fontSize: 0.9rem;
                            color: #666;
                            marginBottom: 4px;
                            width: 100%;",
                    {props.update_time} "更新"
                }
                div {
                    style: "fontSize: 0.9rem;
                            color: #666;
                            marginBottom: 8px;
                            width: 100%;",
                    if !props.update_count.is_empty() {
                        "更新至第" {props.update_count} "集",
                    } else {
                        "暂无更新信息"
                    }
                }
                div {
                    style: "fontSize: 1rem;
                            color: #888;
                            alignItems: center;
                            gap: 8;",
                    span {
                        style: "padding: 2px 8px;
                                background: #f5f5f5;
                                borderRadius: 4px;",
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
            style: "display: block;
                    width: 92.8px;
                    height: 150px;
                    borderRadius: 8px;
                    overflow: hidden,
                    boxShadow: 0 2px 4px rgba(0,0,0,0.1);"
        }
    }
}