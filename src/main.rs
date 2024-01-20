#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use autoflp_desktop::get_people;

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
struct PeopleProps {
    people: Vec<String>,
}

struct SelectedPerson(String);

fn main() {
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let mut people = get_people();

    let names = people.iter().map(|x| {
        let mut first = (&x.0).to_string();
        let last = (&x.1);

        (first + ", " + last).trim().to_uppercase()

    }).collect::<Vec<String>>();
    // names.insert(0, "Select".to_string());

    use_shared_state_provider(cx, || SelectedPerson("".to_string()));

    cx.render(rsx!(
        img {
            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
            class: "primary_button",
            width: "100px"
        },
        About {},
        PeopleList {people: names},
        PeopleDisplay {},
    ))
}

pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(p {
        b {"Dioxus Labs"}
        " An Open Source project dedicated to making Rust UI wonderful!"
    }))
}

pub fn PeopleList(cx: Scope<PeopleProps>) -> Element {

    let selected_person_context = use_shared_state::<SelectedPerson>(cx).unwrap();


    cx.render(rsx!(select {
        onchange: move | event | selected_person_context.write().0 = event.value.clone(),
        cx.props.people.clone().into_iter().map(|x| rsx!{ option { " {x}" }} )
    }))
}

pub fn PeopleDisplay(cx: Scope) -> Element {
    let selected_person_context = use_shared_state::<SelectedPerson>(cx).unwrap();

    cx.render(rsx!(p {
        "{selected_person_context.read().0}"
    }))

}