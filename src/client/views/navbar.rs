use crate::client::components::theme::ThemeToggle;
use dioxus::prelude::*;
use crate::client::components::{Search};
use crate::client::route::Route;

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

                div {
                    class: "nav-left",
                    input {
                        id: "nav-toggle",
                        class: "nav-toggle",
                        r#type: "checkbox"
                    }
                    label {
                        r#for: "nav-toggle",
                        class: "nav-toggle-label",
                        "☰"
                    }

                    nav { 
                        class: "nav-collapse",
                        
                        Link { to: Route::Home {}, "Home" }
                        Link { to: Route::Favorite { id: 1 }, "Favorite" }
                        Link { to: Route::History { id: 1 }, "History" }
                    }
                }

                div { 
                    class: "nav-right",
                    div { class: "search",
                        Search {
                            on_search: move |q| props.on_search.call(q),
                        }
                    }
                    // Theme toggle button placeholder
                    ThemeToggle{}
                }
            }
        }
    }
}
