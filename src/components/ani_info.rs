use dioxus::prelude::*;

#[derive(Clone)]
struct AniData {
    title: String,
    url:   String,
}

#[derive(Props, PartialEq, Clone)]
struct AniProps {
    #[props(into)]
    title: String,
    #[props(into)]
    url:   String,
}

#[component]
fn AniItem(props: AniProps) -> Element {
    let AniProps { title, url } = props;
    rsx! {
        a { href: "{url}", "{title}" }
    }
}

#[component]
pub fn AniList() -> Element {
    let items = vec![
        AniData { title: "📚 Learn Dioxus".into(), url: "https://dioxuslabs.com/learn/0.6/".into() },
        AniData { title: "🚀 Awesome Dioxus".into(), url: "https://dioxuslabs.com/awesome".into() },
        AniData { title: "📡 Community Libraries".into(), url: "https://github.com/dioxus-community/".into() },
        AniData { title: "⚙️ Dioxus Development Kit".into(), url: "https://github.com/DioxusLabs/sdk".into() },
        AniData { title: "💫 VSCode Extension".into(), url: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus".into() },
        AniData { title: "👋 Community Discord".into(), url: "https://discord.gg/XgGxMSkvUM".into() },
    ];

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
