use dioxus::core::{Element, Scope};
use dioxus::core_macro::component;
use dioxus::hooks::{use_shared_state, use_shared_state_provider};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use account::get_account_people::get_account_people;

use crate::client;
use crate::client::{PeopleProps, Route, SelectedAccount, SelectedDeal};
use crate::client::account::deal_list::DealList;
use crate::lib::account::get_full_name::{full_name_from_person, FullNameFormat};
use crate::lib::database::account;
use crate::lib::database::account::get_account_details::get_account_details;
use crate::lib::database::models::{Person, PersonName};

pub mod deal_viewer;
pub mod deal_list;
pub mod account_form;

#[component]
pub fn AccountPage(cx: Scope) -> Element {
    let people = get_account_people();

    let mut names = people.iter().map(|x| {
        let last = (&x.0).to_string();
        let first = (&x.1).to_string();

        let full_name = full_name_from_person(&PersonName {
            first_name: first,
            last_name: last,
            middle_initial: None,
            name_prefix: None,
            name_suffix: None,
        }, FullNameFormat::LastFirstMiddleSuffix, true);

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

// #[component]
pub fn PeopleList(cx: Scope<PeopleProps>) -> Element {
    let selected_account = use_shared_state::<SelectedAccount>(cx).unwrap();
    let selected_account_id = use_state(cx, || String::new());
    let selected_deal = use_shared_state::<SelectedDeal>(cx).unwrap();

    // Ideally, this would be a use_callback that is called when the id changes. For now, this works.
    // Only fetch the user data when the id changes.
    use_effect(cx, (selected_account_id, ), |(selected_account_id, )| {
        to_owned![selected_deal, selected_account];
        async move {
            let account_details = get_account_details(Some(selected_account_id.get().clone()));
            if let Some(account_details) = account_details {
                let (person, acc, deals) = &account_details;
                let new_account = crate::lib::database::models::Account {
                    id: acc.id.clone(), contact: acc.contact.clone(), date_of_birth: acc.date_of_birth.clone(),
                    license_expiration: acc.license_expiration.clone(), license_number: acc.license_number.clone(),
                    notes: acc.notes.clone(), cosigner: acc.cosigner.clone(),
                    current_standing: acc.current_standing.clone(), date_added: acc.date_added.clone(),
                    date_modified: acc.date_modified.clone(),
                };
                let new_person = Person {
                    first_name: person.first_name.to_string(), last_name: person.last_name.to_string(),
                    middle_initial: person.middle_initial.clone(), name_prefix: person.name_prefix.clone(),
                    name_suffix: person.name_suffix.clone(), address_1: person.address_1.clone(),
                    address_2: person.address_2.clone(), address_3: person.address_3.clone(),
                    city: person.city.clone(), state_province: person.state_province.clone(), zip_postal: person.zip_postal.clone(),
                    zip_4: person.zip_4.clone(), phone_primary: person.phone_primary.clone(),
                    phone_secondary: person.phone_secondary.clone(), phone_tertiary: person.phone_tertiary.clone(),
                    email_primary: person.email_primary.clone(), email_secondary: person.email_secondary.clone(),
                    country: person.country.clone(), id: person.id.clone(),
                };

                selected_account.write().account = new_account;
                selected_account.write().person = new_person;
                if (account_details.2.len() > 0) {
                    selected_account.write().deals = account_details.2.clone();
                    selected_deal.write().id = account_details.2[0].clone().0;
                } else {
                    selected_account.write().deals = vec![];
                    selected_deal.write().id = String::new();
                }

            }
        }
    });

    cx.render(rsx!(
        label { class: "text-xl",
            "Select a person"
            select {
                class: "!text-black text-lg",
                onchange: move |event| {
                    to_owned![selected_account_id, selected_deal];
                    selected_account_id.set(event.value.clone());
                    selected_deal.write().id = String::new();
                },
                cx.props.people.iter().map(|[x, y]| rsx!{ option {
                    class: "bg-slate-800 text-white text-lg",
                    key: "${&y}", id: "${&x}", value: "{&y}", x.clone() }} )
            }
        }
        DealList { id: selected_account.read().account.id.clone() }
    ))
}

#[component]
fn NewProfile(cx: Scope) -> Element {
    render!( p { "New Account" } )
}
