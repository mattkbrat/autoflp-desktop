#![allow(non_snake_case)]

// dx serve --hot-reload --platform desktop

mod lib;
mod tests;

use dioxus::html::geometry::euclid::num::Floor;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use lib::database::models::Account;
use lib::finance::add;
use lib::database::deal::get_details::get_deal_details;
use crate::lib::database::account::get_account_details::get_account_details;
use crate::lib::database::account::get_account_people::get_account_people;
use crate::lib::database::models::PaymentForm;
use crate::lib::database::payment::{add_payment, delete_payment};

type People = Vec<[String; 2]>;

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
pub struct PeopleProps {
    people: People,
}

struct SelectedPerson(String);

struct SelectedDeal(String);

struct SelectedAccount(Option<Account>);


fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="dist/styles.css">"#.to_string()),
    )
}


// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
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

    #[nest("/blog")]
    #[layout(Blog)]
    #[route("/")]
    BlogList {},
    #[route("/blog/:name")]
    BlogPost { name: String },
    #[end_layout]
    #[end_nest]
    #[end_layout]
    #[nest("/myblog")]
    #[redirect("/", || Route::BlogList {})]
    #[redirect("/:name", | name: String | Route::BlogPost { name })]
    #[end_nest]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
// ANCHOR_END: router

#[component]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SelectedDeal("".to_string()));

    render! { Router::<Route> {} }
}


#[component]
fn NavBar(cx: Scope) -> Element {
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
                        Link { to: Route::BlogList {}, "Blog" }
                    }
                    li {
                        Link { to: Route::Account {}, "People" }
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

#[component]
fn AccountPage(cx: Scope) -> Element {
    let people = get_account_people();
    use_shared_state_provider(cx, || SelectedPerson("".to_string()));
    use_shared_state_provider(cx, || SelectedAccount(None));

    let mut names = people.iter().map(|x| {
        let first = (&x.0).to_string();
        let last = &x.1;

        let full_name = (first + ", " + last).trim().to_uppercase();

        [full_name, (&x.2).to_string()]
    }).collect::<People>();

    let new_account: [String; 2] = ["New Account".to_string(), "".to_string()];

    names.insert(0, new_account);

    cx.render(rsx!( PeopleList { people: names } ))
}

#[component]
fn FinancePage(cx: Scope) -> Element {
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


#[component]
fn Home(cx: Scope) -> Element {
    render! { About {} }
}

#[component]
fn Blog(cx: Scope) -> Element {
    render! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}

#[component]
fn Account(cx: Scope) -> Element {
    render! { Outlet::<Route> {} }
}

#[component]
fn People(cx: Scope) -> Element {
    render! {
        AccountPage {}
        Outlet::<Route> {}
    }
}

#[component]
fn BlogList(cx: Scope) -> Element {
    render! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}

#[component]
fn BlogPost(cx: Scope, name: String) -> Element {
    render! { h2 { "Blog Post: {name}" } }
}

#[component]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

#[component]
pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(
        p { class: "text-2xl",

            b { "AutoFLP Desktop" }
            pre { class: "text-sm", "Auto Dealer Management Software for Small, Family Owned Businesses" }
        }
    ))
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

// #[component]
// pub fn PeopleDisplay(cx: Scope) -> Element {
//     let selected_person_context = use_shared_state::<SelectedPerson>(cx).unwrap();
//     // let selected_account_context = use_shared_state::<SelectedAccount>(cx).unwrap();
//     let account = use_state(cx, || DEFAULT_ACCOUNT);
//
//     use_shared_state_provider(cx, || SelectedPerson(Some("".to_string())));
//
//     use_effect(cx, (selected_person_context,), |(selected_person_context,)| {
//         to_owned![account];
//
//         let person_id = selected_person_context.read();
//
//         async move {
//             if (person_id.0.is_some()) {
//                 let user = get_account(person_id.0.unwrap());
//                 if user.is_left() {
//                     account.set(user.left().unwrap().unwrap());
//                 } else {
//                     account.set(DEFAULT_ACCOUNT);
//                 }
//             } else {
//                 account.set(DEFAULT_ACCOUNT);
//             }
//         }
//     });
//
//
//     cx.render(rsx!(p {
//         "{account.license_number}"
//     }))
// }
//
//
//


#[component]
fn NewProfile(cx: Scope) -> Element {
    render!( p { "New Account" } )
}

#[component]
fn DealViewer(cx: Scope, deal_id: String) -> Element {

    // let deal = use_state(cx, || None);
    let error_message = use_state(cx, || "".to_string());
    let deal = use_state(cx, || None);
    let refresh_details = use_state(cx, || false);
    let selected_deal = use_shared_state::<SelectedDeal>(cx).unwrap();

    use_effect(cx, (deal_id, refresh_details), |(deal_id, _, )| {
        to_owned![deal_id, deal];
        async move {
            let deal_details = get_deal_details(Some(deal_id));
            if let Some(deal_details) = deal_details {
                deal.set(Some(deal_details));
            } else {
                deal.set(None);
            }
        }
    });

    if selected_deal.read().0.len() == 0 {
        return render!( p { "NO DEAL!: {selected_deal.read().0} {deal_id} " } );
    }


    if deal.is_none() {
        return render!( p { "No Deal Selected" } );
    }

    let details_ref = deal.as_ref();

    let deal = details_ref.unwrap();

    let (deal, inventory, creditor, payments) = deal;

    let lien = match &deal.lien {
        Some(x) => {
            deal.lien.clone().unwrap()
            // lien.unwrap()
        }
        None => String::from("Cash Deal")
    };

    // let lien = lien.unwrap();

    let make = &inventory.make;
    let pmt = deal.pmt.clone();
    let deal_state = deal.state;
    let default_payment = match pmt {
        Some(pmt) if deal_state == 1 => (pmt.parse::<f32>().unwrap() / 10.0).floor() * 10.0,
        None if deal_state == 1 => "100".parse::<f32>().unwrap(),
        _ => 0.0
    };

    let (state_string, state_class) = match deal.state {
        0 => ("Closed", "text-gray-400"),
        1 => ("Active", "text-green-400"),
        _ => ("Unknown", "text-red-400"),
    };

    let state_string = String::from(state_string);

    render!(

        // div {class: "flex flex-row gap-4",
        div { class: "flex flex-col justify-evenly min-w-1/2 max-w-5/6 bg-gray-700 text-white p-2 gap-4",
            if error_message.len() > 0 {
            render!(p {
                    class: "text-lg text-red-700 bg-red-200",
                    "{error_message}"
            
                })
            }
            div { class: "flex flex-row gap-4 border-b-2 border-blue-500 pb-4",
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
                                amount: &*amount[0].to_string(),
                                date: &*date[0].clone(),
                                deal: &*deal_id[0].clone(),
                            };
                            let pmt_result = add_payment(&pmt);
                            if pmt_result.is_ok() {
                                refresh_details.set(!refresh_details.get());
                                error_message.set(String::new());
                            } else {
                                error_message.set(pmt_result.unwrap_err());
                            }
                        }
                    },
                    input { name: "id", class: "hidden", r#type: "id", value: "{deal.id}" }
                    input { name: "date", r#type: "date", value: "2024-08-02" }
                    input {
                        name: "amount",
                        r#type: "number",
                        value: "{default_payment}",
                        step: "10"
                    }
                    button { r#type: "submit", "Submit" }
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
                    r#type: "button",
                    onclick: move |evt| {
                        to_owned![refresh_details, error_message];
                        let result = delete_payment(&payment.id);
                        if result.is_ok() {
                            // TODO: Add a toast message
                            // Rust toast package: https://crates.io/crates/toast
                            println!("Deleted payment");
                            refresh_details.set(!refresh_details.get());
                            error_message.set(String::new());
                        } else {
                            error_message.set(result.unwrap_err());
                            println!("Error deleting payment");
                        }
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

#[component]
fn DealList(cx: Scope, id: Option<String>) -> Element {
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
            if deal.is_some()  {
                let deal = deal.unwrap();
                let (_, _, deal) = deal;
                if deal.len() > 0 {
                    let deal_id = deal[0].0.clone();
                    if selected_deal.read().0 != deal_id {
                        selected_deal.write().0 = deal_id;
                    }
                }
            }

        }
    });




    use_effect(cx, (selected_deal, ), |_| {
        to_owned![selected_deal, nav];
        async move {
            let is_empty = selected_deal.read().0.is_empty();
            let route = match is_empty {
                false => Route::DealViewer { deal_id: selected_deal.read().0.clone() },
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
                let tab_class = match this_deal.eq(&selected_deal.read().0) {
                true => "underline",
                    false => ""
                };
                render!(div{
                class: "{tab_class} flex flex-col gap-4 items-center",
                onclick: move |_| {
                        let this_deal = this_deal.clone();
                        selected_deal.write().0 = this_deal.clone();
                    },
                span{
                    "{deal.1}"
                },
                span{"{deal.2}"}
            })})
        }
    )
}
