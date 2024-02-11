
mod nav_bar;
mod account;
mod meta;
mod inventory;

use dioxus::hooks::use_shared_state_provider;
use dioxus::prelude::*;
use dioxus::prelude::render;
use dioxus_router::prelude::{Routable, Router};
use dioxus_router::prelude::*;

use account::{Account, deal_viewer, People};
use deal_viewer::DealViewer;
use meta::{Home, PageNotFound};
use nav_bar::NavBar;
use inventory::{InventoryPage};

use crate::lib::database::models::Account;
use crate::lib::finance::add;



// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},

    #[nest("/people")]
    #[layout(People)]
    #[route("/")]
    Account {},
    #[route("/people/:deal_id")]
    DealViewer { deal_id: String },
    #[end_layout]
    #[end_nest]
    #[route("/finance")]
    FinancePage {},

    #[route("/inventory")]
    InventoryPage {},

    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SelectedDeal("".to_string()));

    render! { Router::<Route> {} }
}

pub type People = Vec<[String; 2]>;

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
pub struct PeopleProps {
    people: People,
}

pub struct SelectedPerson(String);

pub struct SelectedDeal(String);

pub struct SelectedAccount(Option<Account>);

#[component]
pub fn FinancePage(cx: Scope) -> Element {
    let amount = use_state(cx, || 0);
    let sum = use_state(cx, || 0);

    use_effect(cx, (amount, ), |(amount, )| {
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
