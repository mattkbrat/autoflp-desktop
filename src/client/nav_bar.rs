use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::client;

use crate::client::{PeopleNamesVec, Route};
use crate::lib::account::get_full_name::{full_name_from_person, FullNameFormat};
use crate::lib::database::account::get_account_people::{AccountPeople, get_account_people};
use crate::lib::database::models::PersonName;
use crate::lib::qotd::Quotable;
use crate::lib::unsplash::structs::UnsplashResults;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    let names = use_shared_state::<PeopleNamesVec>(cx).unwrap();
    let people = use_shared_state::<AccountPeople>(cx).unwrap();

    use_effect(cx, (names, people), |(names, people)| {
        to_owned![names];
        async move {
            if names.read().is_empty() {
                let these_people = match people.read().is_empty() {
                    true => get_account_people(),
                    false => people.read().clone()
                };

                let these_names = these_people.iter().map(|x| {
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
                }).collect::<PeopleNamesVec>();

                *names.write() = these_names;
                if people.read().is_empty() {
                    *people.write() = these_people;
                }
            }
        }
    });

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
                        Link { to: Route::InventoryPage {}, "Inventory" }
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
