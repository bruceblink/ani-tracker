use dioxus::prelude::*;

const FAVOR_CSS: Asset = asset!("/assets/styling/favorite.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FAVOR_CSS }
        h1 { "404 - 页面未找到" }
        p { "您请求的页面不存在：" }
        ul {
            for segment in route {
                li { "{segment}" }
            }
        }
    }

}
