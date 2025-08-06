use crate::Route;
use dioxus::prelude::*;
use crate::components::{AniData, Search};
use crate::components::search::search_server;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Favorite] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let mut query = use_signal(|| String::new());
    let results = use_signal(|| Vec::<AniData>::new());

    use_effect(move || {
        let current_query = query(); // 获取最新搜索词
        let mut results = results.clone();

        spawn(async move {
            let data = search_server(current_query.clone()).await.unwrap_or_default();
            results.set(data);
        });
    });
    
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            class: "header",
            div {
                class: "navbar",

                input {
                    r#type: "checkbox",
                    id: "nav-toggle",
                    class: "nav-toggle"
                }

                label {
                    r#for: "nav-toggle",
                    class: "nav-toggle-label",
                    "☰"
                }

                nav {
                    class: "nav-links",
                    Link { to: Route::Home {}, "Home" }
                    Link { to: Route::Favorite { id: 1 }, "Favorite" }
                    Link { to: Route::History { id: 1 }, "History" }
                    
                    // 搜索按钮放在最后，并靠右
                    Search {
                        on_search: move |new_q| query.set(new_q),
                    }
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Favorite`] component depending on the current route.
        Outlet::<Route> {}
    }
}
