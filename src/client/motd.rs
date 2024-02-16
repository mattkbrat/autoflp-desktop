use std::collections::HashMap;

use dioxus::html::{img, span, style};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};
use crate::client::Error;
use crate::lib::qotd::{fetch_quotable, Quotable};

use crate::lib::unsplash::fetch::fetch_unsplash;
use crate::lib::unsplash::structs::{UnsplashResults, UnsplashResultParsed, UnsplashIndex};


// #[component]
pub fn MessageOfTheDay(cx: Scope) -> Element {

    // TODO: Extract business logic.

    let unsplash = use_shared_state::<UnsplashResults>(cx).unwrap();
    let u = use_state(cx, || UnsplashResultParsed::default());
    let result_index = use_shared_state::<UnsplashIndex>(cx).unwrap();

    let quotable = use_shared_state::<Quotable>(cx).unwrap();

    let mut total_count = 16;

    let fetched_unsplash = use_future(cx, (unsplash), |(unsplash)| async move {
        to_owned![unsplash];
        if unsplash.read().is_empty() {
            let unsplash = fetch_unsplash(&"old+ford,pickup+truck,truck,nature", total_count).await;

            let results = match unsplash.is_ok() {
                true if unsplash.as_ref().unwrap().len() > 0 => {
                    let results = unsplash.clone().unwrap().into_iter().map(|r| r.clone()).collect::<Vec<_>>();
                    let results_length = results.len() as u8;
                    if results_length < total_count {
                        total_count = results_length;
                    }
                    Some(results)
                }
                _ => None,
            };

            let unwrapped = unsplash.unwrap().clone();
            unwrapped
        } else {
            let current = unsplash.read().clone();
            current
        }
        // let unsplash = fetch_unsplash(&"old+ford,pickup+truck,truck,nature", &total_count).await;
        // unsplash
    }).value();

    if fetched_unsplash.is_some() && unsplash.read().is_empty() {
        let unwrapped = fetched_unsplash.unwrap().clone();
        *unsplash.write() = unwrapped.clone();
    }


    let quote  = use_future(cx, (quotable), |q| async move {

        to_owned![q];

        let current = q.read().clone();

        if !current.id.is_empty() {
            return current;
        }

        let qotd = fetch_quotable().await;
        println!("Qotd: {:?}", qotd.clone());
        if qotd.is_ok() {
            let quote = qotd.clone();
            let quote = quote.clone().unwrap();
            *q.write() = quote.clone();
            quote
        } else {
            println!("Qotd not fetched");
            Quotable::default()
        }
    }).value();

    let q = match quote {
        Some(x) => x.clone(),
        None => Quotable::default()
    };




    use_effect(cx, (result_index), |(result_index)| {
        to_owned![u, result_index, unsplash];
        async move {
            if unsplash.read().is_empty() {
                return;
            }

            let results = unsplash.clone();

            if *result_index.read() > total_count - 1 {
                *result_index.write() = 0;
                return;
            } else if *result_index.read() < 0 {
                *result_index.write() = total_count - 1;
                return;
            }

            let image = results.read()[(*result_index.read() as usize)].clone();
            let alt_description = image.alt_description.clone();
            let color = image.color.clone();
            let height = image.height;
            let width = image.width;
            let image_url = image.urls.regular.clone();
            let user_link = image.user.links.html.clone();
            let user_name = image.user.name.clone();
            let image_html = image.links.html.clone();

            let new_u = UnsplashResultParsed {
                alt_description: alt_description.unwrap_or(String::new()),
                color,
                height,
                width,
                image_url,
                user_link,
                user_name,
                image_html,
            };

            u.set(new_u.clone());
        }
    });

    //   $: backgroundStyle = `background-color: ${image?.color || "gray"}; max-height: min(${image?.height+'px' || '90dvh'}, 90dvh); max-width: min(${image?.width+'px' || '90dvw'}, 90dvw')`;
    render! {
        div { class: "flex flex-col items-center justify-center relative",
            img {
                src: "{u.image_url}",
                width: "auto",
                height: "auto",
                class: "shadow-inner p-4 outline outline-tertiary-100 block mx-auto my-auto select-none",
                background: "white",
                max_width: "90vw",
                max_height: "90vh",
                title: "{u.alt_description}"
            }
            div {
                class: "absolute flex flex-row items-center justify-center gap-2 top-0 right-0 p-4",
                // style: style! {
                //     "background-color": "rgba(255, 255, 255, 0.5)",
                //     "border-radius": "0.5rem",
                //     "color": "black",
                // },
                background: "rgba(255, 255, 255, 0.5)",
                border_radius: "0.5rem",
                color: "black",
                p { class: "text-xs",
                    a { href: "{u.image_html}", "Photo" }
                    " by "
                    a { href: "{u.user_link}", "{u.user_name}" }
                    " on "
                    a { href: "https://unsplash.com/", "Unsplash" }
                }
                button {
                    class: "btn !text-sm p-4",
                    onclick: move |_| {
                        to_owned![result_index];
                        *result_index.write() += 1;
                    },
                    "Next"
                }
            }
            // Qotable quote
            div {
                class: "absolute flex flex-col items-center justify-center gap-2 bottom-0 left-0 p-4",
                background: "rgba(255, 255, 255, 0.5)",
                border_radius: "0.5rem",
                color: "black",
                a { href: "https://quotable.io/",

                    span { class: "text-sm", "{q.content}" }
                    br {}
                    span { class: "text-xs", "{q.author}" }
                }
            }
        }
    }
}
