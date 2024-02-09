use dioxus::core_macro::component;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(
        p { class: "text-2xl",

            b { "AutoFLP Desktop" }
            pre { class: "text-sm", "Auto Dealer Management Software for Small, Family Owned Businesses" }
        }
    ))
}


#[component]
pub fn Home(cx: Scope) -> Element {
    render! { About {} }
}

#[component]
pub fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
