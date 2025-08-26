use dioxus::prelude::*;
use crate::client::components::ani_list::AniData;
const SEARCH_CSS: Asset = asset!("/assets/styling/search.css");


#[derive(Props, PartialEq, Clone)]
pub struct SearchProps {
    pub on_search: EventHandler<String>,
}

/// Search component that demonstrates fullstack server functions.
#[component]
pub fn Search(props: SearchProps) -> Element {
    // 拷贝 prop 里的回调（EventHandler 实现了 Clone）
    let on_search = props.on_search.clone();
    
    let mut expanded = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: SEARCH_CSS }

        div {
            class: "search-wrap",
            if *expanded.read() {  // 读取 signal 的值要用 read()

                input {
                    class: "search-input expanded",
                    r#type: "text",
                    placeholder: "Search...",
                    autofocus: "autofocus",
                    oninput: move |e| on_search.call(e.value().to_string()),
                    // 写入 signal 用 write()
                    onblur: move |_| *expanded.write() = false,
                }
                button { class: "search-close", onclick: move |_| *expanded.write() = false, "✕" }

            } else {

                button {
                    class: "search-btn",
                    r#type: "button",               // 防止在 form 中触发 submit
                    aria_label: "Open search",      // 无障碍
                    onclick: move |_| *expanded.write() = true,
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "11", cy: "11", r: "6" }
                        path { d: "M21 21l-4.35-4.35" }
                    }
                }

            }
        }
    }
}

// Server functions let us define public APIs on the server that can be called like a normal async function from the client.
// Each server function needs to be annotated with the `#[server]` attribute, accept and return serializable types, and return
// a `Result` with the error type [`ServerFnError`].
//
// When the server function is called from the client, it will just serialize the arguments, call the API, and deserialize the
// response.
#[server(endpoint = "search")]
pub async fn search(input: String) -> Result<Vec<AniData>, ServerFnError> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    let items = vec![
        AniData {
            detail_url: "https://mikanani.me/Home/Bangumi/3582".into(),
            image_url: "https://mikanani.me/images/Bangumi/202504/f86c46e8.jpg?width=400&height=400&format=webp".into(),
            platform: "mikanani".into(),
            title: "安妮·雪莉".into(),
            update_count: "".into(),
            update_info: "2025/07/12 更新".into(),
            update_time: "2025/07/12".into()
        },
        AniData {
            detail_url: "https://mikanani.me/Home/Bangumi/3637".into(),
            image_url: "https://mikanani.me/images/Bangumi/202504/060a6b96.jpg?width=400&height=400&format=webp".into(),
            platform: "mikanani".into(),
            title: "真･武士传 剑勇传说".into(),
            update_count: "".into(),
            update_info: "2025/07/12 更新".into(),
            update_time: "2025/07/12".into()
        },
        AniData {
            detail_url: "https://mikanani.me/Home/Bangumi/3666".into(),
            image_url: "https://mikanani.me/images/Bangumi/202507/2bdcc7f0.jpg?width=400&height=400&format=webp".into(),
            platform: "mikanani".into(),
            title: "更衣人偶坠入爱河 第二季".into(),
            update_count: "".into(),
            update_info: "2025/07/12 更新".into(),
            update_time: "2025/07/12".into()
        },
        AniData {
            detail_url: "https://mikanani.me/Home/Bangumi/3672".into(),
            image_url: "https://mikanani.me/images/Bangumi/202507/f9163e66.jpg?width=400&height=400&format=webp".into(),
            platform: "mikanani".into(),
            title: "怪兽8号 第二季".into(),
            update_count: "".into(),
            update_info: "2025/07/12 更新".into(),
            update_time: "2025/07/12".into()
        },
        AniData {
            detail_url: "https://mikanani.me/Home/Bangumi/3683".into(),
            image_url: "https://mikanani.me/images/Bangumi/202507/57ec5a0a.jpg?width=400&height=400&format=webp".into(),
            platform: "mikanani".into(),
            title: "和雨和你".into(),
            update_count: "".into(),
            update_info: "2025/07/12 更新".into(),
            update_time: "2025/07/12".into()
        },
    ];
    let filtered_ani = items.iter()
        .filter(|item| item.title.contains(&input))
        .cloned()
        .collect::<Vec<AniData>>();
    Ok(filtered_ani)
}
