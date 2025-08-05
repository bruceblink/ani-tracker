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
            class: "ani-item",
            id: "ani-item-{title}",
            style: "border: 1px solid #ccc; 
                    padding: 10px; 
                    margin-bottom: 10px; 
                    display: flex;
                    align-items: center;",
            
            //a { href: "{detail_url}", "{title}" }
            img {
                src: "{image_url}",
                alt: "{title}",
                style: "width: 92.7px; height: 150px; object-fit: cover;",
                class: "ani-item-image"
            }
            h3 { "{title}" }
        }

    }
}