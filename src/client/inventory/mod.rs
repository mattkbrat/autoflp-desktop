use chrono::Datelike;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use inventory::get_inventory_by_id::get_inventory_by_id;

use crate::client::Error;
use crate::lib::database::inventory;
use crate::lib::database::inventory::get_inventory::get_inventory;
use crate::lib::database::inventory::upsert::upsert_inventory;
use crate::lib::database::models::{Inventory, SanitizedInventory};
use crate::lib::date::get_today::get_today;
use crate::lib::inventory::nhtsa::get_vehicle_info;
use crate::lib::titlecase::string_to_title;

type FetchedVin = String;


#[component]
pub fn InventoryPage(cx: Scope) -> Element {
    let id = use_state(cx, || String::new());
    let all_inventory = use_state(cx, || None);
    let selected_inventory = use_state(cx, || SanitizedInventory::default());
    let inventory_state = use_state(cx, || 0);
    let formatted = use_state(cx, || String::new());
    let error = use_shared_state::<Error>(cx).unwrap();
    let vin_fetched = use_state(cx, || FetchedVin::default());


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

    use_effect(cx, (selected_inventory, all_inventory), |(selected_inventory, all_inventory)| {
        //     If the vin is entered manually, find the vehicle info.
        to_owned![selected_inventory, formatted, all_inventory];
        async move {
            let all_inventory_is_some = all_inventory.get().is_some();
            let bad_selected = selected_inventory.make.is_empty();
            let has_valid_vin_length = selected_inventory.vin.len() == 17;
            if all_inventory_is_some && bad_selected && has_valid_vin_length {
                let all_inventory = all_inventory.get().clone().unwrap();
                let vin = selected_inventory.vin.to_lowercase();
                let recorded = all_inventory.into_iter().filter(|x| x.vin.to_lowercase() == vin).collect::<Vec<_>>();
                if (!recorded.is_empty()) {
                    let first = recorded.first().unwrap();
                    let sanitized = Inventory::sanitize(&first).clone();
                    selected_inventory.set(sanitized.clone());
                    formatted.set(string_to_title(&*SanitizedInventory::format(&sanitized)));
                }
            }
        }
    });

    use_effect(cx, (vin_fetched, selected_inventory), |(vin_fetched, selected_inventory)| {
        to_owned![vin_fetched, selected_inventory, formatted, error];
        async move {
            let current_fetched = vin_fetched.get().clone();
            if current_fetched != selected_inventory.vin && selected_inventory.vin.len() == 17 {
                let fetched = get_vehicle_info(selected_inventory.vin.clone()).await;
                if fetched.is_ok() {
                    vin_fetched.set(selected_inventory.vin.clone());
                    let vehicle = fetched.unwrap();
                    let current_selected = selected_inventory.current();

                    let new_inventory = SanitizedInventory {
                        id: current_selected.id.clone(),
                        vin: current_selected.vin.clone(),
                        make: vehicle.make.clone(),
                        model: vehicle.model.clone(),
                        year: vehicle.year.clone(),
                        color: current_selected.color.clone(),
                        fuel: current_selected.fuel.clone(),
                        cwt: current_selected.cwt.clone(),
                        mileage: current_selected.mileage.clone(),
                        date_modified: Option::from(get_today().to_string()),
                        cash: current_selected.cash.clone(),
                        credit: current_selected.credit.clone(),
                        down: current_selected.down.clone(),
                        body: current_selected.body.clone(),
                        state: current_selected.state.clone(),
                    };

                    formatted.set(new_inventory.format().clone());

                    selected_inventory.set(new_inventory);

                    error.write().code = 0;
                } else {
                    let error_message = fetched.unwrap_err();
                    error.write().code = 5002;
                    error.write().message = error_message;
                }
            }
        }
    });

    let handle_upsert = move |i: SanitizedInventory| {
        cx.spawn({
            to_owned![error, i, selected_inventory, all_inventory, formatted];
            // to_owned![error, i, all_inventory];
            async move {
                // Parse the form

                // let account_string = SelectedAccount::details(selected_account.read().clone());
                let result = upsert_inventory(i.clone()).await;

                if result.is_ok() {
                    error.write().code = 0;

                    let vin = i.vin.clone().to_lowercase();
                    let mut cloned = all_inventory.get().clone().unwrap();
                    let found = cloned.iter().position(|x| x.vin.to_lowercase() == vin);
                    if found.is_some() {
                            let found = found.unwrap();
                            let result = result.unwrap();
                            cloned[found] = result;
                            all_inventory.set(Some(cloned));
                        } else {
                        let result = result.unwrap();
                        cloned.push(result);
                        all_inventory.set(Some(cloned));

                    }
                    {
                        let mut new_default = SanitizedInventory::default();
                        new_default.vin = vin;
                        selected_inventory.set(new_default);
                        formatted.set(String::new());
                    }



                } else {
                    error.write().code = 5001;
                    error.write().message = result.unwrap_err();
                }
            }
        });
    };

    // let handle_lookup = move |_| {
    //     cx.spawn({
    //         to_owned![error, selected_inventory, inventory_state, all_inventory];
    //         async move {
    //             let fetched = get_vehicle_info(selected_inventory.vin.clone()).await;
    //             if fetched.is_ok() {
    //                 vin_fetched.set(selected_inventory.vin.clone());
    //                 let vehicle = fetched.unwrap();
    //                 let current_selected = selected_inventory.current();
    //                 let with_lookup = selected_inventory.get().clone().with_lookup(vehicle);
    //                 selected_inventory.set(with_lookup);
    //                 formatted.set(selected_inventory.current().format());
    //                 error.write().code = 0;
    //             } else {
    //                 let error_message = fetched.unwrap_err();
    //                 error.write().code = 5002;
    //                 error.write().message = error_message;
    //             }
    //         }
    //     });
    // };


    let all = all_inventory.as_ref();

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
                            let formatted = string_to_title(&*Inventory::format(x));
                            let id = x.vin.clone();
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
                        to_owned![id, selected_inventory, formatted];
                        id.set(String::new());
                        selected_inventory.set(SanitizedInventory::default());
                        formatted.set(String::new());
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
                to_owned![selected_inventory, inventory_state];
                let values = &event.data.values;
                let deal_id = values.get("id");
                let id = selected_inventory.id.clone();
                let values = &event.data.values;
                let current = SanitizedInventory {
                    id,
                    vin: values.get("vin").unwrap()[0].to_string(),
                    make: values.get("make").unwrap()[0].to_string(),
                    model: values.get("model").unwrap()[0].to_string(),
                    year: values.get("year").unwrap()[0].to_string(),
                    color: values.get("color").unwrap()[0].to_string(),
                    fuel: values.get("fuel").unwrap()[0].to_string(),
                    cwt: values.get("cwt").unwrap()[0].to_string(),
                    mileage: values.get("mileage").unwrap()[0].to_string(),
                    date_modified: Option::from(get_today().to_string()),
                    cash: values.get("cash").unwrap()[0].to_string(),
                    credit: values.get("credit").unwrap()[0].to_string(),
                    down: values.get("down").unwrap()[0].to_string(),
                    body: values.get("body").unwrap()[0].to_string(),
                    state: inventory_state.get().clone(),
                };
                handle_upsert(current.clone());
            },
            div {
                class: "col-span-full grid-cols-2 gap-4",
                label { class: "flex flex-col uppercase",
                "VIN"
                input {
                    name: "vin",
                    r#type: "text",
                    onchange: move |event| {
                        to_owned![selected_inventory, vin_fetched];
                        let mut current_selected = selected_inventory.get().clone();
                        current_selected.vin = event.value.clone();
                        selected_inventory.set(current_selected.clone());
                        vin_fetched.set(FetchedVin::default());
                    },
                    value: "{selected_inventory.vin}"
                }
            },
                // button {
                //     r#type: "button",
                //     onclick: handle_lookup
                // }
            },

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