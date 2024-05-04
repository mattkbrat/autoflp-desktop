use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::client::account::account_form::AccountForm;
use crate::client::account::deal_list::DealListTabs::{TabAccountForm, TabDealList};
use crate::client::account::NewProfile;
use crate::client::Route::Account;
use crate::client::{Route, SelectedAccount, SelectedDeal, SelectedDealDetails};
use crate::lib::account::get_full_name::full_name_from_person;
use crate::lib::account::get_full_name::FullNameFormat::LastFirstMiddleSuffix;
use crate::lib::database::account::get_account_details::get_account_details;
use crate::lib::database::deal::DealsByAccount;
use crate::lib::database::models::{Account as NewAccount, Person, PersonName};

#[derive(Debug, Clone, PartialEq)]
enum DealListTabs {
    TabAccountForm,
    TabDealList,
}

#[component]
pub fn DealList(cx: Scope, id: Option<String>) -> Element {
    let nav = use_navigator(cx);
    let selected_deal = use_shared_state::<SelectedDeal>(cx).unwrap();
    let selected_account = use_shared_state::<SelectedAccount>(cx).unwrap();
    let selected_tab = use_state(cx, || TabDealList);

    // let (person, account, deals) = use_memo(cx, (details,), |(details,)| {
    //     to_owned![details];
    //     // async move {
    //     //         let details = details.get().as_ref();
    //     //         details.unwrap().clone()
    //     //     }

    //     details.get().as_ref().clone().unwrap()
    // });

    use_effect(cx, (selected_deal, selected_tab), |(_, tab)| {
        to_owned![selected_deal, selected_account, nav, selected_tab];

        async move {
            let is_empty = selected_deal.read().id.is_empty();
            let tab = tab.get();

            let route = match is_empty {
                false if tab == &TabDealList => Route::DealViewer {
                    deal_id: selected_deal.read().id.clone(),
                    account: selected_account.read().clone()
                },
                _ => Account {},
            };

            nav.replace(route)
        }
    });

    #[component]
    fn DealListContent(cx: Scope, deals: DealsByAccount) -> Element {
        let nav = use_navigator(cx);
        let selected_deal = use_shared_state::<SelectedDeal>(cx).unwrap();
        let selected_account = use_shared_state::<SelectedAccount>(cx).unwrap();
        let selected_deal_id = selected_deal.read().id.clone();

        println!(
            "deal list {} {}",
            deals.len(),
            selected_account.read().account.id
        );

        render!(deals.into_iter().map(|deal| {
            to_owned![deal];
            let (this_deal, this_date, this_make, state) = deal;
            let this_date = this_date.split(" ").into_iter().next().unwrap(); // "2021-08-01 00:00:00"
            let state_class = match state.eq(&1) {
                true => "!text-green-200",
                _ => "",
            };
            let tab_class = match this_deal.eq(&selected_deal_id) {
                true => format!("selected {}", state_class).to_string(),
                false => "".to_string(),
            };
            render!(div{
                class: "{tab_class} flex flex-col items-center uppercase",
                onclick: move |_| {
                        let this_deal = this_deal.clone();
                        selected_deal.write().id = this_deal;
                    },
                span{
                    class: "text-sm",
                    "{this_date}"
                },

                span{
                    class: "{state_class}" ,
                    "{this_make}"
                }
            })
        }))
    }

    let selected = selected_account.read();
    let deals = selected.deals.clone();

    let current_tab = selected_tab.get();

    let selected_tab_class = "selected".to_string();

    let account_form_tab_class = (match current_tab == &TabAccountForm {
        true => selected_tab_class.clone(),
        false => "".to_string(),
    })
    .to_string();

    let deal_view_tab_class = (match current_tab == &TabDealList {
        true => selected_tab_class.clone(),
        false => "".to_string(),
    })
    .to_string();

    render!(
        div { class: "flex flex-col gap-4",
            div { class: "tabs", id: "deal-list-tabs",
                div {
                    class: "{account_form_tab_class}",
                    onclick: move |_| {
                        selected_tab.set(TabAccountForm);
                    },
                    span { "Account" }
                }
                div {
                    class: "{deal_view_tab_class}",
                    onclick: move |_| {
                        selected_tab.set(TabDealList);
                    },
                    span { "Deals" }
                }
            }

            // Deal List
            div { class: "flex flex-row gap-4 items-center h-min", id: "tab-content",
                match current_tab {
                    &TabAccountForm => render!( AccountForm {} ),
                    &TabDealList => render!( div {class: "tabs", DealListContent { deals: deals.clone() }} ),
                }
            }
        }
    )
}
