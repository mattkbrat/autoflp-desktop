use dioxus::hooks::{to_owned, use_effect, use_shared_state, use_state};
use dioxus::prelude::*;
use crate::client::{Error, SelectedAccount};
use crate::lib::database::account::update_person::update_details;
use crate::lib::database::models::{Account, PaymentForm, Person, PersonForm};
use crate::lib::database::payment::add_payment;

#[component]
pub fn AccountForm(cx: Scope) -> Element {

    let selected_account = use_shared_state::<SelectedAccount>(cx).unwrap();
    let error = use_shared_state::<Error>(cx).unwrap();

    let person = &selected_account.read().person;
    let account = &selected_account.read().account;

    let handle_update_account = move |person: PersonForm| {
        cx.spawn({
            to_owned![error, person, selected_account];
            async move {
                // let account_string = SelectedAccount::details(selected_account.read().clone());
                let result = update_details(person.clone()).await;
                if result.is_ok() {
                    error.write().code = 0;
                    selected_account.write().person = PersonForm::to_person(person);
                } else {
                    error.write().code = 2001;
                    error.write().message = result.unwrap_err();
                }
            }
        });
    };


    let prefix = match &person.name_prefix {
        Some(prefix) => prefix.to_owned(),
        None => "".to_owned()
    };

    let suffix = match &person.name_suffix {
        Some(suffix) => suffix.to_owned(),
        None => "".to_owned()
    };

    let middle_initial = match &person.middle_initial {
        Some(middle) => middle.to_owned(),
        None => "".to_owned()
    };

    let address_2 = match &person.address_2 {
        Some(address) => address.to_owned(),
        None => "".to_owned()
    };

    let address_3 = match &person.address_3 {
        Some(address) => address.to_owned(),
        None => "".to_owned()
    };

    let zip_4 = match &person.zip_4 {
        Some(zip) => zip.to_owned(),
        None => "".to_owned()
    };

    let phone_secondary = match &person.phone_secondary {
        Some(phone) => phone.to_owned(),
        None => "".to_owned()
    };

    let phone_tertiary = match &person.phone_tertiary {
        Some(phone) => phone.to_owned(),
        None => "".to_owned()
    };

    let email_primary = match &person.email_primary {
        Some(email) => email.to_owned(),
        None => "".to_owned()
    };

    let email_secondary = match &person.email_secondary {
        Some(email) => email.to_owned(),
        None => "".to_owned()
    };

    render!(

        form {
            class: "grid grid-cols-5 gap-4 uppercase text-left",
            onsubmit: move |event| {
                to_owned![error, selected_account];
                let values = &event.data.values;
                let person_form = PersonForm {
                    id: values.get("id").unwrap()[0].to_string(),
                    name_prefix: values.get("prefix").unwrap()[0].to_string(),
                    first_name: values.get("first").unwrap()[0].to_string(),
                    middle_initial: values.get("middle").unwrap()[0].to_string(),
                    last_name: values.get("last").unwrap()[0].to_string(),
                    name_suffix: values.get("suffix").unwrap()[0].to_string(),
                    address_1: values.get("address_1").unwrap()[0].to_string(),
                    address_2: values.get("address_2").unwrap()[0].to_string(),
                    address_3: values.get("address_3").unwrap()[0].to_string(),
                    city: values.get("city").unwrap()[0].to_string(),
                    state_province: values.get("state").unwrap()[0].to_string(),
                    zip_postal: values.get("zip").unwrap()[0].to_string(),
                    zip_4: values.get("zip_4").unwrap()[0].to_string(),
                    phone_primary: values.get("phone_primary").unwrap()[0].to_string(),
                    phone_secondary: values.get("phone_secondary").unwrap()[0].to_string(),
                    phone_tertiary: values.get("phone_tertiary").unwrap()[0].to_string(),
                    email_primary: values.get("email_primary").unwrap()[0].to_string(),
                    email_secondary: values.get("email_secondary").unwrap()[0].to_string(),
                    country: "United States".to_string(),
                };
                handle_update_account(person_form.clone());
            },
            input {
                name: "id",
                class: "hidden",
                r#type: "text",
                value: "{&person.id}"
            }

            label { class: "flex flex-col",
                "Prefix"
                input {
                    name: "prefix",
                    class: "uppercase",
                    r#type: "text",
                    value: "{prefix}"
                }
            }
            label { class: "flex flex-col",
                "First Name"
                input {
                    name: "first",
                    class: "uppercase",
                    r#type: "text",
                    value: "{person.first_name}"
                }
            }
            label { class: "flex flex-col",
                "Middle Initial"
                input {
                    class: "uppercase",
                    name: "middle",
                    r#type: "text",
                    value: "{middle_initial}"
                }
            }
            label { class: "flex flex-col",
                "Last Name"
                input {
                    name: "last",
                    class: "uppercase",
                    r#type: "text",
                    value: "{person.last_name}"
                }
            }
            label { class: "flex flex-col",
                "Suffix"
                input {
                    name: "suffix",
                    class: "uppercase",
                    r#type: "text",
                    value: "{suffix}"
                }
            }

            // Full-width label
            label { class: "flex flex-col col-span-full",
                "Address (L1)"
                input {

                    // Full-width label
                    name: "address_1",

                    // Full-width label
                    class: "uppercase",

                    // Full-width label
                    r#type: "text",

                    // Full-width label
                    value: "{person.address_1}"
                }
            }

            div { class: "col-span-full grid grid-cols-2 gap-4",
                label { class: "flex flex-col",
                    "Address (L2)"
                    input {
                        name: "address_2",
                        class: "uppercase",
                        r#type: "text",
                        value: "{address_2}"
                    }
                }
                label { class: "flex flex-col",
                    "Address (L3)"
                    input {
                        name: "address_3",
                        class: "uppercase",
                        r#type: "text",
                        value: "{address_3}"
                    }
                }
            }

            div { class: "col-span-full grid grid-cols-2 gap-4",
                div { class: "grid grid-cols-2 gap-2",
                    label { class: "flex flex-col",
                        "City"
                        input {
                            name: "city",
                            class: "uppercase",
                            r#type: "text",
                            value: "{person.city}"
                        }
                    }
                    label { class: "flex flex-col",
                        "State"
                        input {
                            name: "state",
                            class: "uppercase",
                            r#type: "text",
                            value: "{person.state_province}"
                        }
                    }
                }
                div { class: "flex col-span-1 gap-2",
                    label { class: "flex flex-col flex-1",
                        "Zip"
                        input {
                            name: "zip",
                            class: "uppercase w-full",
                            r#type: "text",
                            value: "{person.zip_postal}"
                        }
                    }
                    label { class: "flex flex-col",
                        "+4"
                        input {
                            name: "zip_4",
                            class: "uppercase w-16",
                            r#type: "text",
                            value: "{zip_4}"
                        }
                    }
                }
            }
            div { class: "col-span-full grid grid-cols-3 gap-4",
                label { class: "flex flex-col ",
                    "Primary Phone"
                    input {
                        name: "phone_primary",
                        class: "uppercase",
                        r#type: "text",
                        value: "{person.phone_primary}"
                    }
                }
                label { class: "flex flex-col ",
                    "Secondary Phone"
                    input {
                        name: "phone_secondary",
                        class: "uppercase",
                        r#type: "text",
                        value: "{phone_secondary}"
                    }
                }
                label { class: "flex flex-col ",
                    "Tertiary Phone"
                    input {
                        name: "phone_tertiary",
                        class: "uppercase",
                        r#type: "text",
                        value: "{phone_tertiary}"
                    }
                }
            }
            div { class: "col-span-full grid grid-cols-2 gap-4",
                label { class: "flex flex-col ",
                    "Primary Email"
                    input {
                        name: "email_primary",
                        class: "uppercase",
                        r#type: "email",
                        value: "{email_primary}"
                    }
                }
                label { class: "flex flex-col ",
                    "Secondary Email"
                    input {
                        name: "email_secondary",
                        class: "uppercase",
                        r#type: "email",
                        value: "{email_secondary}"
                    }
                }
            }

            button {
                class: "btn-success w-1/2 mx-auto col-span-full",
                r#type: "submit",
                "Submit"
            }
        }
    )
}