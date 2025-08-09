use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;

use crate::views::{Favorite, History, Home, Navbar, NotFound};
/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the route to work.
///
/// Each variant represents a different URL pattern that can be matched by the route. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(AppLayout)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/favorite/:id")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Favorite { id: i32 },

        #[route("/history/:id")]
        History { id: i32 },
        // Dioxus 会自动将最后一个带有 /:.. 路径的路由作为回退路由。
        #[route("/:..route")]
        NotFound {
            route: Vec<String>,
        }
}


#[component]
pub fn AppLayout() -> Element {
    let mut query = use_signal(|| "".to_string());
    provide_context(query.clone());  // 👈 将 query提示为全局 提供给子组件
    rsx! {
        div {
            class: "App",
            Navbar {
                on_search: move |s| query.set(s),
            }

            // 包裹所有页面并传递 query
            Outlet::<Route> {}
        }

    }
}