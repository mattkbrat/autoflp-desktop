use crate::lib::database::models::Inventory;
use crate::lib::titlecase::string_to_title;

pub fn format_inventory(i: &Inventory, titlecase: bool) -> String {
    let make = match &i.make == "none" {
        true => "",
        false => &i.make
    };

    let model = match &i.model {
        Some(x) if x != "none" => x.to_owned(),
        _ => String::new()
    };

    let year = match &i.year == "none" {
        true => String::new(),
        false => {
            let year = i.year.to_string();
            let year_split = year.split(".");
            year_split.clone().nth(0).unwrap().to_string()
        }
    };

    let color = match &i.color {
        Some(x) if x != "none" => x.to_owned(),
        _ => String::new()
    };

    let vin_last_four = match &i.vin.len() > &4 {
        true => i.vin.chars().skip(i.vin.len() - 4).collect::<String>(),
        false => i.vin.to_owned()
    };

    let inv = format!("{} {} {} {} {}", color, year, make, model, vin_last_four);

    if titlecase {
        return string_to_title(&inv);
    }

    let formatted = format!("{} {} {} {} {}", color, year, make, model, vin_last_four);
    formatted.trim().to_string().to_uppercase()
}
