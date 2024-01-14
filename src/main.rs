use autoflp_desktop::{establish_connection, get_account};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_clicks(ui.get_clicks() + 5);
    });


    let account = get_account();

    ui.set_name(account.first_name.into());

    ui.run()

}
