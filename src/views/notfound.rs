use dioxus::prelude::*;

const APP_CSS: Asset = asset!("/assets/styling/app.css");

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APP_CSS }
        div {
            class: "not-found",
            h1 { "404 - 页面未找到" }
            p { "您请求的页面不存在：" }
            ul {
                for segment in route {
                    "{segment}"
                }
            }
        }

    }

}
