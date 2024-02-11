use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use crate::lib::database;
use crate::lib::database::models::Inventory;
use crate::lib::database::schema::inventory;

pub fn get_inventory_by_id(inventory_id: Option<&String>) -> Option<(Inventory)> {

    if inventory_id.is_none() {
        return None;
    };

    let this_id = inventory_id.unwrap();

    if this_id.len() <= 0 {
        return None;
    }

    let mut conn = database::establish_connection();

    let selected_inventory = inventory::table
        .find(this_id)
        .select(Inventory::as_select())
        .load::<Inventory>(&mut conn);

    selected_inventory.unwrap().into_iter().nth(0)
}
