use chrono::Datelike;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use inventory::get_inventory_by_id::get_inventory_by_id;
use crate::client::Error;

use crate::lib::database::inventory;
use crate::lib::database::inventory::get_inventory::get_inventory;
use crate::lib::database::models::{Inventory, SanitizedInventory};
use crate::lib::date::get_today::get_today;
use crate::lib::inventory::nhtsa::get_vehicle_info;
use crate::lib::titlecase::string_to_title;

#[derive(Debug)]
struct VinFetched {
    vin: String,
    fetched: bool,
}

impl Default for VinFetched {
    fn default() -> Self {
        Self {
            vin: String::new(),
            fetched: false,
        }
    }
}


#[component]
pub fn InventoryPage(cx: Scope) -> Element {
    let id = use_state(cx, || String::new());
    let all_inventory = use_state(cx, || None);
    let selected_inventory = use_state(cx, || SanitizedInventory::default());
    let inventory_state = use_state(cx, || 1);
    let formatted = use_state(cx, || String::new());
    let error = use_shared_state::<Error>(cx).unwrap();
    let vin_fetched = use_state(cx, || VinFetched::default());


    use_effect(cx, inventory_state, |state| {
        to_owned![all_inventory, formatted, selected_inventory];
        async move {
            let inventory = get_inventory(state.get());
            all_inventory.set(Some(inventory));
            formatted.set(String::new());
            selected_inventory.set(SanitizedInventory::default());
        }
    });

    use_effect(cx, (id, ), |(id, )| {
        to_owned![selected_inventory, formatted];
        async move {
            let inventory = get_inventory_by_id(Some(&(id.get())));
            if let Some(inventory) = inventory {
                formatted.set(string_to_title(&*Inventory::format(&inventory)));
                selected_inventory.set(Inventory::sanitize(&inventory));
            } else {
                formatted.set(String::new());
                selected_inventory.set(SanitizedInventory::default());
            }
        }
    });


    let all = all_inventory.as_ref();

    //         model -> Nullable<Text>,
    //         body -> Nullable<Text>,
    //         color -> Nullable<Text>,
    //         fuel -> Nullable<Text>,
    //         cwt -> Nullable<Text>,
    //         mileage -> Nullable<Text>,
    //         date_added -> Nullable<Text>,
    //         date_modified -> Nullable<Text>,
    //         picture -> Nullable<Text>,
    //         cash -> Nullable<Text>,
    //         credit -> Nullable<Text>,
    //         down -> Nullable<Text>,

    let (next_state, next_state_string) = match inventory_state.get() {
        0 => (1, "Open"),
        _ => (0, "Closed"),
    };

    let this_year = get_today().year();


    cx.render(rsx!(
        div { class: "flex flex-row gap-4",
            if let Some(all) = all {
            rsx!(label {
                "Select inventory",
                select {
                    class: "!text-black text-lg h-fit w-full",
                    onchange: move |event| {
                        id.set(event.value.clone()) ;
                    },
                    optgroup{
                        label: "Select inventory",
                        class: "bg-slate-800 text-white !text-2xl",
                        option {
                            key: "0", id: "0", value: "0", "New Inventory Record"
                        },
                        all.iter().map(|x| {
                            let formatted = string_to_title(&*Inventory::format(x.to_owned()));
                            let id = x.vin.to_owned();
                            rsx!{ option {
                            key: "{id}", id: "{id}", value: "{x.id}", "{formatted}" }} }
                        )
                    }
                }
            }, button {
                    r#type: "button",
                    class: "btn-success p-4 text-lg outline-secondary-400",
                    onclick: move |_| {
                        to_owned![inventory_state];
                        inventory_state.set(next_state);
                    },
                    "{next_state_string}"
            }, button {
                r#type: "button",
                class: "btn-warning p-4 outline-secondary-400",
                onclick: move |_| {
                        to_owned![id];
                        id.set(String::new());
                },
                "Clear"
            },
            
            )
            }
        }
        if formatted.get().len() > 0 {
            rsx!(div{"{formatted}"})
        }
        form {
            class: "grid grid-cols-3 gap-4 uppercase text-left",
            onsubmit: move |event| {
                let values = &event.data.values;
                let deal_id = &values.get("id");
                println!("{:?}", values);
            },
            label { class: "flex flex-col uppercase",
                "VIN"
                input { name: "vin", r#type: "text", value: "{vin_fetched.vin}" }
            }
            label { class: "flex flex-col uppercase",
                "Make"
                input { name: "make", r#type: "text", value: "{selected_inventory.make}" }
            }
            label { class: "flex flex-col uppercase",
                "Model"
                input { name: "model", r#type: "text", value: "{selected_inventory.model}" }
            }
            label { class: "flex flex-col uppercase",

                "Year"
                input {
                    name: "year",
                    r#type: "number",
                    step: "1",
                    min: "1900",
                    max: "{this_year}",
                    value: "{selected_inventory.year}"
                }
            }
            label { class: "flex flex-col uppercase",

                "Color"
                input { name: "color", r#type: "text", value: "{selected_inventory.color}" }
            }
            label { class: "flex flex-col uppercase",

                "Mileage"
                input { name: "mileage", r#type: "text", value: "{selected_inventory.mileage}" }
            }
            label { class: "flex flex-col uppercase",

                "Fuel"
                input { name: "fuel", r#type: "text", value: "{selected_inventory.fuel}" }
            }
            label { class: "flex flex-col uppercase",

                "CWT"
                input { name: "cwt", r#type: "text", value: "{selected_inventory.cwt}" }
            }
            label { class: "flex flex-col uppercase",

                "Body"
                input { name: "body", r#type: "text", value: "{selected_inventory.body}" }
            }
            // label {
            //
            //     "Picture",
            //     input {
            //
            //         r#type: "text",
            //         value: "{selected_inventory.picture}",
            //     }
            // },
            label { class: "flex flex-col uppercase",
                "Cash"
                input {
                    name: "cash",
                    r#type: "number",
                    step: "10",
                    value: "{selected_inventory.cash}"
                }
            }
            label { class: "flex flex-col uppercase",
                "Credit"
                input {
                    name: "credit",
                    r#type: "number",
                    step: "10",
                    value: "{selected_inventory.credit}"
                }
            }
            label { class: "flex flex-col uppercase",
                "Down"
                input {
                    name: "down",
                    r#type: "number",
                    step: "10",
                    value: "{selected_inventory.down}"
                }
            }
            button { class: "btn-success w-1/2 mx-auto col-span-full", r#type: "submit", "Submit" }
        }
    ))
}