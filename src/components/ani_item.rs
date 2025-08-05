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
            style: "border:1px solid #ccc; padding:10px; margin-bottom:10px; display:flex; align-items:center;",
            // 直接把整个 props（按值）展开给 AniInfo
            AniInfo { ..info_props }
        }
    }
}


#[component]
pub fn AniInfo(props: AniProps) -> Element {
    let AniProps {
        title,
        detail_url,
        update_count,
        update_info,
        image_url,
        update_time,
        platform,
    } = props;

    rsx! {
        div {
            class: "ani-info",
            id: "ani-info-{title}",
            style: "padding: 10px; margin-bottom: 10px;",

            h3 { "{title}" }
            p { "Update Count: {update_count}" }
            p { "Update Info: {update_info}" }
            p { "Update Time: {update_time}" }
            p { "Platform: {platform}" }
            a { href: "{detail_url}", "More Details" }
        }
    }
}