#![allow(non_snake_case)]

// dx serve --hot-reload --platform desktop

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use autoflp_desktop::{finance, get_account_details, get_account_people};
use autoflp_desktop::models::Account;
use finance::add;

type People = Vec<[String; 2]>;

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
pub struct PeopleProps {
    people: People,
}

struct SelectedPerson(Option<String>);

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
    #[route("/people")]
    PeoplePage {},
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
                        Link { to: Route::PeoplePage {}, "People" }
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
fn PeoplePage(cx: Scope) -> Element {
    let people = get_account_people();
    use_shared_state_provider(cx, || SelectedPerson(Option::from("".to_string())));
    use_shared_state_provider(cx, || SelectedAccount(None));

    let names = people.iter().map(|x| {
        let first = (&x.0).to_string();
        let last = &x.1;

        let full_name = (first + ", " + last).trim().to_uppercase();

        [full_name, (&x.2).to_string()]
    }).collect::<People>();

    cx.render(rsx!( PeopleList { people: names } ))
}

#[component]
fn FinancePage(cx: Scope) -> Element {

    let amount = use_state(cx, || 0);
    let sum = use_state(cx, || 0);

    use_effect(cx, (amount,), |(amount,)| {
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

pub fn PeopleList(cx: Scope<PeopleProps>) -> Element {
    let selected_person_context = use_shared_state::<SelectedPerson>(cx).unwrap();

    cx.render(rsx!(
        select { onchange: move |event| selected_person_context.write().0 = Option::from(event.value.clone()),
            cx.props.people.iter().map(|[x, y]| rsx!{ option { key: "${&y}", id: "${&x}", value: "{&y}", x.clone() }} )
        }
        Profile { id: selected_person_context.read().0.clone().expect("MUST HAVE ID") }
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
fn Profile(cx: Scope, id: Option<String>) -> Element {
    let details = use_state(cx, || None);

    // Only fetch the user data when the id changes.
    use_effect(cx, (id, ), |(id, )| {
        to_owned![details];
        async move {
            let account_details = get_account_details(id);
            if let Some(account_details) = account_details {
                details.set(Some(account_details));
            }
        }
    });

    if details.is_none() {
        return render!( p { "Loading..." } );
    }

    let details = details.clone();


    let details_ref = details.as_ref();


    let (deals, person, account) = details_ref.unwrap();
    let deals = deals.as_ref();
    let deal_is_some = deals.is_some();

    render!(
        div {
            h1 { "{person.first_name} {person.last_name}" }
            p { "{account.license_number}" }
            if deal_is_some {
                let deals_vec = deals.unwrap().clone();
                render!(
                // expected `Vec<(Deal, Inventory)>`, but found `&Vec<(Deal, Inventory)>`
            
            render!(
            
            ul {
            deals_vec.iter().map(|((deal, inventory, lien ), payments)| {
            let lien = deal.lien.clone();
            let make = inventory.make.clone();
            let (state_string, state_class) = match deal.state {
                0 => ("Closed", "text-gray-400"),
                1 => ("Active", "text-green-400"),
                _ => ("Unknown", "text-red-400"),
            };
            if let Some(lien) =  lien {
            render!(
            li {
                                            span {
class: state_class,
                                            "{state_string}"
                                            },
                                             "{lien} {make}" },
                                        ul {
            payments.iter().map(|payment| {
                                                render!(
                                                li { "{payment.amount} {payment.date}" }
                                                )
                                        })
                                        }
            )
            } else {
            render!(
            li { "{deal.state}" }
            )
            }
            })
            }
            )
                )
            } else {
                render!(
            
                p { "No deals" }
                )
            }
        }
    )
}
