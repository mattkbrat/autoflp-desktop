use dioxus::core::{Element, Scope};
use dioxus::core_macro::component;
use dioxus::hooks::{use_shared_state, use_shared_state_provider};
use dioxus::html::geometry::euclid::num::Floor;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use account::get_account_people::get_account_people;

use crate::client;
use crate::client::{PeopleProps, Route, SelectedAccount, SelectedDeal, SelectedPerson};
use crate::client::account::deal_list::DealList;
use crate::lib::database::account;

pub mod deal_viewer;
pub mod deal_list;

#[component]
pub fn AccountPage(cx: Scope) -> Element {
    let people = get_account_people();
    use_shared_state_provider(cx, || SelectedPerson("".to_string()));
    use_shared_state_provider(cx, || SelectedAccount(None));

    let mut names = people.iter().map(|x| {
        let first = (&x.0).to_string();
        let last = &x.1;

        let full_name = (first + ", " + last).trim().to_uppercase();

        [full_name, (&x.2).to_string()]
    }).collect::<client::People>();

    let new_account: [String; 2] = ["New Account".to_string(), "".to_string()];

    names.insert(0, new_account);

    cx.render(rsx!( PeopleList { people: names } ))
}

#[component]
pub fn People(cx: Scope) -> Element {
    render! {
        AccountPage {}
        Outlet::<Route> {}
    }
}

#[component]
pub fn Account(cx: Scope) -> Element {
    render! { Outlet::<Route> {} }
}

#[component]
pub fn PeopleList(cx: Scope<PeopleProps>) -> Element {
    let selected_person_context = use_shared_state::<SelectedPerson>(cx).unwrap();
    let selected_deal = use_shared_state::<SelectedDeal>(cx).unwrap();

    cx.render(rsx!(
        label { class: "dark-select",
            "Select a person"
            select {
                onchange: move |event| {
                    selected_person_context.write().0 = event.value.clone();
                    selected_deal.write().0 = String::new();
                },
                cx.props.people.iter().map(|[x, y]| rsx!{ option { key: "${&y}", id: "${&x}", value: "{&y}", x.clone() }} )
            }
        }
        DealList { id: selected_person_context.read().0.clone() }
    ))
}

#[component]
fn NewProfile(cx: Scope) -> Element {
    render!( p { "New Account" } )
}
