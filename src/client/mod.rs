
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
    use_shared_state_provider(cx, || SelectedDeal::default());

    render! { Router::<Route> {} }
}

pub type People = Vec<[String; 2]>;

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
pub struct PeopleProps {
    people: People,
}

pub struct SelectedPerson(String);

// deal.id, (full_name, inventory_string)
pub struct SelectedDeal((String, (String, String)));

impl Default for SelectedDeal {
    // call with `SelectedDeal::default()`
    fn default() -> Self {
        SelectedDeal((String::new(), (String::new(), String::new())))
    }
}

impl SelectedDeal {
    // call with `SelectedDeal::account_details()`
    pub fn account_details(&self) -> String {
        format!("{} ({})", self.0.1.0, self.0.1.1).to_uppercase()
    }
}

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
