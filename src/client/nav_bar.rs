use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::client::Route;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            div { class: "flex items-center gap-4",
                div { class: "flex flex-row items-center",

                    img {
                        src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                        class: "primary_button",
                        width: "100px"
                    }
                }

                ul { class: "flex justify-around w-screen h-min",

                    li {
                        Link { to: Route::Home {}, "Home" }
                    }
                    li {
                        Link { to: Route::Account {}, "People" }
                    }
                    li {
                        Link { to: Route::FinancePage {}, "Finance" }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
