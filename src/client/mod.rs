
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
use crate::lib::account::get_full_name::{full_name_from_person, FullNameFormat};
use crate::lib::database::deal::DealsByAccount;

use crate::lib::database::models::{Account, Person, PersonName};
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
    use_shared_state_provider(cx, || SelectedAccount::default());
    use_shared_state_provider(cx, || Error::default());

    render! { Router::<Route> {}, ErrorDisplay{ } }
}

#[component]
fn ErrorDisplay(cx: Scope) -> Element {
    let error = use_shared_state::<Error>(cx).unwrap();
    let error_code = error.read().code.clone();

    if error_code == 0 {
        return render!(rsx! { div {} });
    }


    let error_message = error.read().message.clone();

    cx.render(rsx! {
        div {
            class: "error",
            span {
                class: "text-2xl",
                "Error"
            }
            span { "{error_code}" }
            span { class: "text-wrap", "{error_message}" }
        }
    })
}

pub type People = Vec<[String; 2]>;

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
pub struct PeopleProps {
    people: People,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct SelectedDealDetails {
    inventory_string: String,
    open: bool,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct SelectedDeal{
    id: String,
    details: SelectedDealDetails
}

impl Default for SelectedDeal {
    // call with `SelectedDeal::default()`
    fn default() -> Self {
        SelectedDeal {
            id: String::new(),
            details: SelectedDealDetails {
                inventory_string: String::new(),
                open: false,
            }
        }
    }
}

impl SelectedDeal {
    // call with `SelectedDeal::account_details()`
    pub fn details(self) -> String {
        format!("{}", self.details.inventory_string).to_uppercase()
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct SelectedAccount{account: Account, person: Person, deals: DealsByAccount}

impl Default for SelectedDealDetails {
    // call with `SelectedDeal::default()`
    fn default() -> Self {
        SelectedDealDetails {
            inventory_string: String::new(),
            open: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct Error{code: u32, message: String}

impl Default for Error {
    fn default() -> Self {
        Error{code: 0, message: String::new()}
    }
}

impl Error {
    pub fn details(self) -> String {
        format!("Error {}: {}", self.code, self.message).to_uppercase()
    }
}

impl Default for SelectedAccount {
    // call with `SelectedDeal::default()`
    fn default() -> Self {
        SelectedAccount{account: Account::default(), person: Person::default(), deals: vec![]}
    }
}

impl SelectedAccount {
    // call with `SelectedDeal::account_details()`
    pub fn full_name(&self) -> String {
        format!("{}", full_name_from_person(&PersonName{
            first_name: self.person.first_name.to_string(),
            last_name: self.person.last_name.to_string(),
            middle_initial: None,
            name_prefix: None,
            name_suffix: None,
        }, FullNameFormat::LastFirstMiddleSuffix, true)).to_uppercase()
    }

    pub fn details(self) -> String {
        format!("{}", &self.full_name()).to_uppercase()
    }
}


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
