
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::lib::database;
use crate::lib::database::models::Inventory;
use crate::lib::database::schema::inventory;

pub fn get_inventory(state: &i32) -> Vec<Inventory> {

    let mut conn = database::establish_connection();

    let all_inventory = inventory::table
        .select(Inventory::as_select())
        .filter(inventory::state.eq(state))
        .order_by(inventory::make.asc())
        .then_order_by(inventory::model.asc())
        .then_order_by(inventory::year.desc())
        .load(&mut conn);

    all_inventory.unwrap()
}
