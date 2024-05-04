use diesel::dsl::now;
use dioxus::core::{Element, Scope};
use dioxus::core_macro::component;
use dioxus::hooks::{to_owned, use_effect, use_shared_state, use_state};
use dioxus::prelude::*;

use deal::get_details::get_deal_details;
use models::PaymentForm;
use payment::add_payment;

use crate::client::{Error, SelectedAccount, SelectedDeal};
use crate::lib::database::payment::delete_payment;
use crate::lib::database::{deal, models, payment};
use crate::lib::date::get_today::get_today_string;

#[component]
pub fn DealViewer(cx: Scope, deal_id: String, account: SelectedAccount) -> Element {
    // let deal = use_state(cx, || None);
    let deal = use_state(cx, || None);
    let refresh_details = use_state(cx, || false);
    let selected_deal = use_shared_state::<SelectedDeal>(cx).unwrap();
    let error = use_shared_state::<Error>(cx).unwrap();

    let handle_record_payment = move |pmt: PaymentForm| {
        cx.spawn({
            to_owned![refresh_details, error, account];
            async move {
                let account_string = SelectedAccount::details(&account);
                let pmt_result = add_payment(pmt, account_string.to_owned()).await;
                if pmt_result.is_ok() {
                    refresh_details.set(!refresh_details.get());
                } else {
                    error.write().code = 0;
                    // error_message.set(pmt_result.unwrap_err());
                    error.write().code = 5001;
                    error.write().message = pmt_result.unwrap_err();
                }
            }
        });
    };

    let handle_delete_payment = move |id: String| {
        cx.spawn({
            to_owned![refresh_details, error, selected_deal, account];
            async move {
                let account_string = SelectedAccount::details(&account);
                let result = delete_payment(&id, account_string.to_owned()).await;
                if result.is_ok() {
                    refresh_details.set(!refresh_details.get());
                    // error_message.set(String::new());
                    error.write().code = 0;
                } else {
                    // error_message.set(result.unwrap_err());
                    // println!("Error deleting payment");
                    error.write().code = 5002;
                    error.write().message = result.unwrap_err();
                };
            }
        });
    };

    use_effect(
        cx,
        (selected_deal, refresh_details),
        |(selected_deal, _)| {
            to_owned![deal, deal_id];
            async move {
                let deal_details = get_deal_details(Some(selected_deal.read().id.clone()));
                if let Some(deal_details) = deal_details {
                    deal.set(Some(deal_details));
                } else {
                    deal.set(None);
                }
            }
        },
    );

    if selected_deal.read().id.is_empty() {
        return render!( p { "Select a Deal" } );
    }

    if deal.is_none() {
        return render!( p { "No Deal Selected" } );
    }

    let details_ref = deal.as_ref();

    let deal = details_ref.unwrap();

    let (deal, inventory, _creditor, payments) = deal;

    let lien = match &deal.lien {
        Some(_) => deal.lien.clone().unwrap(),
        None => String::from("Cash Deal"),
    };

    // let lien = lien.unwrap();

    let make = &inventory.make;
    let pmt = deal.pmt.clone();
    let deal_state = deal.state;
    let default_payment = match pmt {
        Some(pmt) if deal_state == 1 => (pmt.parse::<f32>().unwrap() / 10.0).floor() * 10.0,
        None if deal_state == 1 => "100".parse::<f32>().unwrap(),
        _ => 0.0,
    };

    // let account_string = format!("{:?}", deal.account);

    let (state_string, state_class) = match deal.state {
        0 => ("Closed", "text-warning-400"),
        1 => ("Active", "text-success-400"),
        _ => ("Unknown", "text-error-400"),
    };

    let state_string = String::from(state_string);
    let account_string = use_memo(cx, account, |account| SelectedAccount::details(&account));
    let inventory_string = SelectedDeal::details(selected_deal.read().clone());

    let today = get_today_string();

    render!(

        // div {class: "flex flex-row gap-4",
        div { class: "flex flex-col justify-evenly min-w-1/2 max-w-5/6 bg-surface-900 text-surface-200 p-2 gap-4",
            p {
            }
            h2 { class: "text-3xl underline", "{account_string} {inventory_string}" }
            div { class: "flex flex-row gap-4 border-b-2 border-primary-500 pb-4",
                span { class: "{state_class} font-bold", "{state_string}" }
                span { "{lien}" }
                span { "{make}" }
            }

            div { class: "flex flex-row" }
            div { class: "grid grid-cols-3 flex-row w-max gap-4 text-left",
                span { class: "w-full", "Date" }
                span { class: "w-full", "Amount" }
                span { class: "w-full", "Action" }

                if deal_state == 1 {
                render!(form {
                    class: "contents",
                    onsubmit: move |event| {
                        let values = &event.data.values;
                        let deal_id = &values.get("id");
                        let date = &values.get("date");
                        let amount = &values.get("amount");
                        if let (Some(date), Some(deal_id), Some(amount)) = (date, deal_id, amount) {
                            let pmt = PaymentForm {
                                amount: amount[0].to_string(),
                                date: date[0].clone(),
                                deal: deal_id[0].clone(),
                            };
                            handle_record_payment(pmt);
                        }
                    },
                    input { name: "id", class: "hidden", r#type: "id", value: "{deal.id}" }
                    input { name: "date", r#type: "date", value: "{today}" }
                    input {
                        name: "amount",
                        r#type: "number",
                        value: "{default_payment}",
                        step: "10"
                    }
                    button {
                            class: "btn-success",
                            r#type: "submit", "Submit" }
                })}
                payments.iter().map(|payment| {
                    render!(
                    span {
                    "{payment.date}"
                    },
                    span {
                    "{payment.amount}"
                    },
                    button {
                    class: "btn-warning",
                    r#type: "button",
                    onclick: move |_| {
                        handle_delete_payment(payment.id.clone());
                    },
                    "Delete"
                    }
                    )
                    }
                )
            }
        }
    )
}
