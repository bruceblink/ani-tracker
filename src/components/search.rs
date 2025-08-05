use dioxus::prelude::*;
use crate::components::ani_list::AniData;
const SEARCH_CSS: Asset = asset!("/assets/styling/search.css");


#[derive(Props, PartialEq)]
#[derive(Clone)]
pub struct SearchProps {
    pub on_search: EventHandler<String>,
}

/// Search component that demonstrates fullstack server functions.
#[component]
pub fn Search(props: SearchProps) -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside other hooks, async blocks, if statements, or loops.
    //
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.
    let SearchProps { on_search } = props;

    rsx! {
        document::Link { rel: "stylesheet", href: SEARCH_CSS }

        div {
            id: "search",
            h4 {
                class: "search-title",
                style: "color: #333; font-weight: bold; margin-bottom: 10px; text-align: center;",
                "Search" 
            }
            input {
                placeholder: "Type here to search...",
                // `oninput` is an event handler that will run when the input changes. It can return either nothing or a future
                // that will be run when the event runs.
                oninput: move |e| on_search.call(e.value().to_string()),
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
#[server]
pub async fn search_server(input: String) -> Result<Vec<AniData>, ServerFnError> {
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
