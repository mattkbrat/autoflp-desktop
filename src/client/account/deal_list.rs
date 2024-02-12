use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::client::{Route, SelectedDeal};
use crate::client::account::NewProfile;
use crate::lib::account::get_full_name::full_name_from_person;
use crate::lib::account::get_full_name::FullNameFormat::LastFirstMiddleSuffix;
use crate::lib::database::account::get_account_details::get_account_details;
use crate::lib::database::models::PersonName;

#[component]
pub fn DealList(cx: Scope, id: Option<String>) -> Element {
    let nav = use_navigator(cx);
    let selected_deal = use_shared_state::<SelectedDeal>(cx).unwrap();

    let details = use_state(cx, || None);
    let is_new_account = use_state(cx, || true);

    // let redirect = || { nav.replace(Route::DealViewer { deal_id: selected_deal.read().0.clone() }); };

    // Ideally, this would be a use_callback that is called when the id changes. For now, this works.
    // Only fetch the user data when the id changes.
    use_effect(cx, (id, ), |(id, )| {
        to_owned![details, is_new_account];
        async move {
            let account_details = get_account_details(id);
            if let Some(account_details) = account_details {
                details.set(Some(account_details));
                is_new_account.set(false);
            } else {
                is_new_account.set(true);
            }
        }
    });

    use_effect(cx, (details, ), |(details, )| {
        to_owned![details, selected_deal];
        async move {
            let deal = details.get().as_ref();
            if deal.is_some() {
                let deal = deal.unwrap();
                let (person, _, deal) = deal;
                if deal.len() > 0 {
                    let first_deal = &deal[0];
                    let deal_id = first_deal.0.clone();
                    if selected_deal.read().0.0 != deal_id {

                        let name = PersonName {
                            first_name: person.first_name.to_string(),
                            last_name: person.last_name.to_string(),
                            middle_initial: person.middle_initial.clone(),
                            name_prefix: None,
                            name_suffix: person.name_suffix.clone(),
                        };

                        let full_name = full_name_from_person(&name, LastFirstMiddleSuffix, true);

                        // println!("{:?} {:?}", full_name, first_deal);

                        selected_deal.write().0 = (deal_id, (full_name, first_deal.2.clone()));
                    }
                }
            }
        }
    });

    // let (person, account, deals) = use_memo(cx,  (details,), |(details,)| {
    //     to_owned![details];
    //     // async move {
    //     //         let details = details.get().as_ref();
    //     //         details.unwrap().clone()
    //     //     }
    //
    //     details.get().as_ref().clone().unwrap()
    //
    // });


    use_effect(cx, (selected_deal, ), |_| {
        to_owned![selected_deal, nav];
        async move {
            let is_empty = selected_deal.read().0.0.is_empty();
            let route = match is_empty {
                false => Route::DealViewer { deal_id: selected_deal.read().0.0.clone() },
                _ => Route::Account {}
            };

            nav.replace(route)
        }
    });

    if details.is_none() || *is_new_account.get() {
        return render!( NewProfile {} );
    }

    let details = details.clone();


    let details_ref = details.as_ref();


    let (person, account, deals) = details_ref.unwrap();



    render!(
        div { class: "flex flex-row gap-4 items-center h-min",
            deals.into_iter().map(|deal| {
                to_owned![deal];
                let this_deal = deal.0;
                let tab_class = match this_deal.eq(&selected_deal.read().0.0) {
                true => "underline",
                    false => ""
                };
                render!(div{
                class: "{tab_class} flex flex-col gap-4 items-center",
                onclick: move |_| {
                        let this_deal = this_deal.clone();
                        selected_deal.write().0 = (this_deal.clone(), (String::new(), String::new()));
                    },
                span{
                    "{deal.1}"
                },
                span{"{deal.2}"}
            })})
        }
    )
}
