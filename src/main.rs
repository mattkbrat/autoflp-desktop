use std::rc::Rc;
use slint::{ModelRc, SharedString, VecModel};
use autoflp_desktop::{establish_connection, get_account, get_people};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_clicks(ui.get_clicks() + 5);
    });


    let account = get_account();

    let people = get_people();


    let mut vec = vec!["Hello".into()];

    people.iter().for_each(|p| vec.push(p.into()));

    let vec_model = VecModel::from(vec);


    let the_model : Rc<VecModel<SharedString>> = Rc::new(vec_model);

    ui.set_name(account.first_name.into());
    ui.set_names(ModelRc::from(the_model.clone()));

    ui.run()

}
