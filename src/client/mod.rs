mod account;
mod finance;
mod inventory;
mod meta;
mod motd;
mod nav_bar;

use dioxus::hooks::use_shared_state_provider;
use dioxus::prelude::render;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_router::prelude::{Routable, Router};

use crate::lib;
use crate::lib::account::get_full_name::{full_name_from_person, FullNameFormat};
use crate::lib::database::account::get_account_people::{get_account_people, AccountPeople};
use crate::lib::database::deal::DealsByAccount;
use account::{deal_viewer, Account, People};
use deal_viewer::DealViewer;
use finance::page::FinancePage;
use inventory::InventoryPage;
use meta::{Home, PageNotFound};
use nav_bar::NavBar;

use crate::lib::database::models::{Account, Person, PersonName};
use crate::lib::finance::add;
use crate::lib::qotd::{fetch_quotable, Quotable, Root};
use crate::lib::unsplash::fetch::fetch_unsplash;
use crate::lib::unsplash::structs::{Root2, UnsplashIndex, UnsplashResultParsed, UnsplashResults};

pub type PeopleNamesVec = Vec<[String; 2]>;
pub type PeopleVec = Vec<[String; 2]>;

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
    DealViewer { deal_id: String, account: SelectedAccount},
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
    use_shared_state_provider(cx, || UnsplashResults::default());
    use_shared_state_provider(cx, || Quotable::default());
    use_shared_state_provider(cx, || UnsplashIndex::default());
    use_shared_state_provider(cx, || PeopleNamesVec::new());
    use_shared_state_provider(cx, || AccountPeople::new());

    render! {
        Router::<Route> {}
        ErrorDisplay {}
    }
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
        div { class: "error",
            span { class: "text-2xl", "Error" }
            span { "{error_code}" }
            span { class: "text-wrap", "{error_message}" }
        }
    })
}

// Remember: Owned props must implement `PartialEq`!

#[derive(Clone, Debug, PartialEq, Props)]
pub struct SelectedDealDetails {
    inventory_string: String,
    open: bool,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct SelectedDeal {
    id: String,
    details: SelectedDealDetails,
}

impl Default for SelectedDeal {
    // call with `SelectedDeal::default()`
    fn default() -> Self {
        SelectedDeal {
            id: String::new(),
            details: SelectedDealDetails {
                inventory_string: String::new(),
                open: false,
            },
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
pub struct SelectedAccount {
    account: Account,
    person: Person,
    deals: DealsByAccount,
}

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
pub struct Error {
    code: u32,
    message: String,
}

impl Default for Error {
    fn default() -> Self {
        Error {
            code: 0,
            message: String::new(),
        }
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
        SelectedAccount {
            account: Account::default(),
            person: Person::default(),
            deals: vec![],
        }
    }
}

impl SelectedAccount {
    // call with `SelectedDeal::account_details()`
    pub fn full_name(&self) -> String {
        format!(
            "{}",
            full_name_from_person(
                &PersonName {
                    first_name: self.person.first_name.to_string(),
                    last_name: self.person.last_name.to_string(),
                    middle_initial: None,
                    name_prefix: None,
                    name_suffix: None,
                },
                FullNameFormat::LastFirstMiddleSuffix,
                true
            )
        )
        .to_uppercase()
    }

    pub fn details(&self) -> String {
        (&self.full_name()).to_string().to_uppercase()
    }
}
