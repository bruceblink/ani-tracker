use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AniProps {
    #[props(into)]
    title: String,
    #[props(into)]
    url:   String,
}

#[component]
pub fn AniItem(props: AniProps) -> Element {
    let AniProps { title, url } = props;
    rsx! {
        a { href: "{url}", "{title}" }
    }
}