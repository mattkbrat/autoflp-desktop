use chrono::NaiveDate;
use diesel::row::NamedRow;
use dioxus::hooks::use_shared_state_provider;
use dioxus::html::p;
use dioxus::prelude::render;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_router::prelude::{Routable, Router};
use crate::client::{Error, SelectedAccount};
use crate::lib::database::account::get_account_details::get_account_details;
use crate::lib::database::account::get_account_people::AccountPeople;
use crate::lib::database::account::get_creditors::get_creditors;
use crate::lib::database::models::Person;
use crate::lib::date::get_today::get_today_string;

use crate::lib::finance::add;
use crate::lib::finance::calc::{
    calculate_finance, Creditor, FinanceCalc, FinanceCalcParams, Prices, Taxes,
};

// #[derive(Debug, Clone)]
// pub(crate) struct FinanceCalc {
//     pub(crate) selling_trade_differential: f64,
//     pub(crate) state_tax_dollar: f64,
//     pub(crate) county_tax_dollar: f64,
//     pub(crate) city_tax_dollar: f64,
//     pub(crate) rtd_tax_dollar: f64,
//     pub(crate) total_tax_dollar: f64,
//     pub(crate) total_tax_percent: f64,
//     pub(crate) cash_balance_with_tax: f64,
//     pub(crate) unpaid_cash_balance: f64,
//     pub(crate) finance_amount: f64,
//     pub(crate) total_loan: f64,
//     pub(crate) deferred_payment: f64,
//     pub(crate) monthly_payment: f64,
//     pub(crate) last_payment: f64,
//     pub(crate) last_payment_due_date: String,
//     pub(crate) first_payment_due_date: String,
//     pub(crate) deferred: f64,
//     pub(crate) total_cost: f64,
// }

#[component]
pub fn FinancePage(cx: Scope) -> Element {

    let selected_account = use_shared_state::<SelectedAccount>(cx).unwrap();
    let error = use_shared_state::<Error>(cx).unwrap();

    let tax = use_state(cx, || Taxes::new());
    let prices = use_state(cx, || Prices::new());
    let creditor = use_state(cx, || Creditor::new());
    let first_payment = use_state(cx, || chrono::offset::Utc::now().date_naive());
    let people = use_shared_state::<AccountPeople>(cx).unwrap();
    let cosigner = use_state(cx, || String::new());
    let creditors = use_state(cx, || get_creditors());

    let selected_account_id = use_state(cx, || String::new());



    // Ideally, this would be a use_callback that is called when the id changes. For now, this works.
    // Only fetch the user data when the id changes.
    use_effect(cx, (selected_account_id, ), |(selected_account_id, )| {
        to_owned![selected_account];
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
            }
        }
    });



    let person_account = &selected_account.read();

    let person = &person_account.person;
    let account = &person_account.account;

    let address = person.address();
    let license = account.license_number.clone();
    let expiration = account.license_expiration.clone().unwrap_or_default();
    let date_of_birth = account.date_of_birth.clone().unwrap_or_default();
    let email = person.email_primary.clone().unwrap_or_default();


    let calculated = use_memo(
        cx,
        (tax, prices, creditor, first_payment),
        move |(tax, prices, creditor, first_payment)| {
            calculate_finance(FinanceCalcParams {
                tax: tax.get().clone(),
                prices: prices.get().clone(),
                creditor: creditor.get().clone(),
                first_payment: first_payment.get().clone(),
            })
        },
    );

    let last_payment_date = calculated.last_payment_due_date.clone();
    let monthly_payment = calculated.monthly_payment.clone();
    let last_payment = calculated.last_payment.clone();
    let lien = calculated.total_loan.clone();
    let financed = calculated.finance_amount.clone();


    // use_effect(cx, (calculated,), |(calculated,)| {
    //     to_owned![calculated];
    //     async move {
    //         println!("{:?}", calculated);
    //     }
    // });

    cx.render(rsx!(
        form {
            class: "grid grid-cols-4 gap-4 uppercase text-left",
            onsubmit: move |event| {
                to_owned![tax, prices, creditor, first_payment];
                let values = &event.data.values;
            },
            label { class: "flex flex-col col-span-3",
                "Date of Deal"
                input {
                    name: "deal_date",
                    value: "{first_payment}",
                    r#type: "date",
                    class: "uppercase",
                    onchange: move |event| {
                        to_owned![first_payment];
                        let value = event.value.clone();
                        let as_date = value.parse().unwrap();
                        first_payment.set(as_date);
                    }
                }
            }
            label { class: "flex flex-col",
                "Term (Months)"
                input {
                    name: "term",
                    value: "{creditor.term}",
                    r#type: "number",
                    step: 1,
                    onchange: move |event| {
                        to_owned![creditor];
                        let mut new = creditor.get().clone();
                        let value = event.value.clone();
                        let as_num: Result<i32, _> = value.parse();
                        new.term = match as_num.is_err() {
                            true => 12,
                            false => as_num.unwrap()
                        };
                        creditor.set(new);
                    }
                }
            }

            label { class: "text-xl col-span-2 flex flex-col",
                "Select a person"
                select {
                    class: "!text-black text-lg uppercase",
                    onchange: move |event| {
                        to_owned![selected_account_id];
                        selected_account_id.set(event.value.clone());
                    },
                    people.read().iter().map(|(last, first, id)| rsx!{ option {
                        class: "bg-slate-800 text-white text-lg",
                        key: "${&id}", id: "${&id}", value: "{&id}", "{last}, {first}" }} )
                }
            }

            label { class: "flex flex-col col-span-2",
                "Cosigner"
                input {
                    name: "tax_state",
                    value: "{cosigner}",
                    r#type: "text",
                    class: "uppercase"
                }
            }
            h2 { class: "text-xl col-span-full", "Account Details" }
            label { class: "flex flex-col",
                "License"
                p { class: "uppercase", "{license}" }
            }
            label { class: "flex flex-col",
                "Expiration"
                p { class: "uppercase", "{expiration}" }
            }
            label { class: "flex flex-col",
                "Address"
                p { class: "uppercase", "{address}" }
            }
            label { class: "flex flex-col",
                "Date of Birth"
                p { class: "uppercase", "{date_of_birth}" }
            }
            label { class: "flex flex-col",
                "Email"
                p { class: "uppercase", "{email}" }
            }

            label { class: "flex flex-col",
                "Primary Phone"
                p { class: "uppercase", "{person.phone_primary}" }
            }

            span { class: "col-span-2" }



            if creditor.term > 0 {
                render!(
            label {
                 class: "flex flex-col col-span-2",
                "Select a creditor"


                      select {
                class: "!text-black text-lg uppercase",
                onchange: move |event| {
                    to_owned![creditor];
                    let value = event.value.clone();
                    let new = creditors.get().iter().find(|c| c.id == value).unwrap();
                    creditor
                        .set(Creditor {
                            filingFees: new.filing_fees.clone().parse().unwrap(),
                            apr: new.apr.clone().parse().unwrap(),
                            term: creditor.term.clone(),
                            id: new.id.clone(),
                        });
                },
                creditors.get().iter().map(|creditor| rsx!{ option {
                        class: "bg-slate-800 text-white text-lg",

                    key: "${&creditor.id}", id: "${&creditor.id}", value: "{&creditor.id}", "{&creditor.business_name}" }} )
            }
            },


                                label { class: "flex flex-col",
                "Filing Fees"
                input {
                    name: "filing_fees",
                    value: "{creditor.filingFees}",
                    r#type: "number",
                    step: 1,
                    onchange: move |event| {
                        to_owned![creditor];
                        let mut new = creditor.get().clone();
                        let value = event.value.clone();
                        let as_num: f64 = value.parse().unwrap();
                        new.filingFees = as_num;
                        creditor.set(new);
                    }
                }
            }
            label { class: "flex flex-col",
                "APR"
                input {
                    name: "apr",
                    value: "{creditor.apr}",
                    r#type: "number",
                    step: 1,
                    onchange: move |event| {
                        to_owned![creditor];
                        let mut new = creditor.get().clone();
                        let value = event.value.clone();
                        let as_num: f64 = value.parse().unwrap();
                        new.apr = as_num;
                        creditor.set(new);
                    }
                }
            },
                )
            }


            label { class: "flex flex-col col-span-2",
                "Selling Price"
                input {
                    name: "selling_price",
                    value: "{prices.selling}",
                    r#type: "number",
                    step: 1,
                    onchange: move |event| {
                        to_owned![prices];
                        let mut new = prices.get().clone();
                        let value = event.value.clone();
                        let as_num: f64 = value.parse().unwrap();
                        new.selling = as_num;
                        prices.set(new);
                    }
                }
            }
            label { class: "flex flex-col col-span",
                "Trade-In Value"
                input {
                    name: "trade_in",
                    value: "{prices.trade}",
                    r#type: "number",
                    step: 1,
                    onchange: move |event| {
                        to_owned![prices];
                        let mut new = prices.get().clone();
                        let value = event.value.clone();
                        let as_num: f64 = value.parse().unwrap();
                        new.trade = as_num;
                        prices.set(new);
                    }
                }
            }
            label { class: "flex flex-col",
                "Down Payment"
                input {
                    name: "down_payment",
                    value: "{prices.down}",
                    r#type: "number",
                    step: 1,
                    onchange: move |event| {
                        to_owned![prices];
                        let mut new = prices.get().clone();
                        let value = event.value.clone();
                        let as_num: f64 = value.parse().unwrap();
                        new.down = as_num;
                        prices.set(new);
                    }
                }
            }
            label { class: "flex flex-col",
                "State Tax (%)"
                input {
                    name: "tax_state",
                    value: "{tax.state}",
                    r#type: "number",
                    step: 0.1,
                    class: "uppercase",
                    onchange: move |event| {
                        to_owned![tax];
                        let mut new_tax = tax.get().clone();
                        let value = event.value.clone();
                        new_tax.state = value.parse().unwrap();
                        tax.set(new_tax);
                    }
                }
            }
            label { class: "flex flex-col",
                "City Tax (%)"
                input {
                    name: "tax_city",
                    value: "{tax.city}",
                    r#type: "number",
                    step: 0.1,
                    onchange: move |event| {
                        to_owned![tax];
                        let mut new_tax = tax.get().clone();
                        let value = event.value.clone();
                        new_tax.city = value.parse().unwrap();
                        tax.set(new_tax);
                    }
                }
            }
            label { class: "flex flex-col",
                "RTD Tax (%)"
                input {
                    name: "tax_rtd",
                    value: "{tax.rtd}",
                    r#type: "number",
                    step: 0.1,
                    onchange: move |event| {
                        to_owned![tax];
                        let mut new_tax = tax.get().clone();
                        let value = event.value.clone();
                        new_tax.rtd = value.parse().unwrap();
                        tax.set(new_tax);
                    }
                }
            }
            label { class: "flex flex-col",
                "County Tax (%)"
                input {
                    name: "tax_county",
                    value: "{tax.county}",
                    r#type: "number",
                    step: 0.1,
                    onchange: move |event| {
                        to_owned![tax];
                        let mut new_tax = tax.get().clone();
                        let value = event.value.clone();
                        new_tax.county = value.parse().unwrap();
                        tax.set(new_tax);
                    }
                }
            }

            section { class: "col-span-full grid grid-cols-5",
            label { class: "flex flex-col",
                "Total Loan"
                p { class: "uppercase", "{lien}" }
            }
            label { class: "flex flex-col",
                "Monthly Payment"
                p { class: "uppercase", "{monthly_payment}" }
            }
            label { class: "flex flex-col",
                "Last Payment"
                p { class: "uppercase", "{last_payment}" }
            }
            label { class: "flex flex-col",
                "Last Payment Due"
                p { class: "uppercase", "{last_payment_date}" }
            }
            label { class: "flex flex-col",
                "Financed Amount"
                p { class: "uppercase", "{financed}" }
            }
            }


        }
    ))
}
