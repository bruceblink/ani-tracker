use crate::Route;
use dioxus::prelude::*;
use crate::components::{Search};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[derive(Props, PartialEq, Clone)]
pub struct NavbarProps {
    on_search: EventHandler<String>,
}
/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Favorite] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar(props: NavbarProps) -> Element {

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
                        on_search: move |query| props.on_search.call(query),
                    }
                }
            }
        }
    }
}
