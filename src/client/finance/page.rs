use dioxus::hooks::use_shared_state_provider;
use dioxus::prelude::render;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_router::prelude::{Routable, Router};

use crate::lib::finance::add;

#[component]
pub fn FinancePage(cx: Scope) -> Element {
    let amount = use_state(cx, || 0);
    let sum = use_state(cx, || 0);

    use_effect(cx, (amount,), |(amount,)| {
        to_owned![sum];
        async move {
            let user = add::add(amount.get(), 2);
            sum.set(user);
        }
    });

    cx.render(rsx!(
        div {
            input {
                value: "{amount}",
                r#type: "number",
                oninput: move |evt| amount.set(evt.value.clone().parse::<u32>().unwrap())
            }
            input {
                value: "{sum}",
                r#type: "number",
                readonly: true,
                oninput: move |evt| amount.set(evt.value.clone().parse::<u32>().unwrap())
            }
        }
    ))
}
