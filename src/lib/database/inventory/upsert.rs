use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::expression_methods::ExpressionMethods;
use crate::lib::database;
use crate::lib::database::models::{Inventory, SanitizedInventory};
use crate::lib::database::schema::inventory;
use crate::lib::database::schema::inventory::vin;
use crate::lib::inventory::nhtsa::get_vehicle_info;

pub(crate) async fn upsert_inventory(mut upsert: SanitizedInventory) -> Result<Inventory, String> {
    let mut conn = database::establish_connection();

    let selected_inventory = inventory::table
        .filter(vin.eq(upsert.vin.clone()))
        .select(Inventory::as_select())
        .first::<Inventory>(&mut conn)
        .optional()
        .unwrap();

    let can_update = selected_inventory.is_some();

    let valid_vin = get_vehicle_info(upsert.vin.clone()).await;

    if valid_vin.is_err() {
        let error_message = valid_vin.unwrap_err().1;
        return Err(format!("Invalid VIN: {}", error_message));
    }

    let result = match (can_update) {
        true => {
            let selected_inventory = selected_inventory.unwrap();
            println!("Selected inventory: {:?}", selected_inventory);
            // if (selected_inventory.expect("MUST HAVE RESULT").is_empty()) {
            //     return Err("Failed to update inventory".to_string());
            // }
            let updated = diesel::update(&selected_inventory)
            .set(&upsert).execute(&mut conn);

            match updated.is_ok() {
                true => Ok("Inventory updated".to_string()),
                false => {
                    let error_message = updated.unwrap_err();
                    Err(format!("Failed to update inventory: {}", error_message).to_string())}
            }
        }
        false => {
            let new_id = uuid::Uuid::new_v4().to_string();
            upsert.id = new_id;
            let inserted_inventory = diesel::insert_into(inventory::table).values(&upsert).execute(
                &mut conn
            );

            match inserted_inventory.is_ok() {
                true => Ok("Inventory created".to_string()),
                false => {
                    let error_message = inserted_inventory.unwrap_err();
                    Err(format!("Failed to create inventory: {}", error_message).to_string())
                }
            }
        }
    };

    if (result.is_ok()) {
        let selected_inventory = inventory::table
            .filter(vin.eq(upsert.vin.clone()))
            .select(Inventory::as_select())
            .first::<Inventory>(&mut conn)
            .optional()
            .unwrap();

        if selected_inventory.is_none() {
            return Err("Failed to retrieve inventory".to_string());
        }

        // TODO: Notify the user of the result.
        return Ok(selected_inventory.unwrap());
    }

    Err(result.unwrap())

}

#[cfg(test)]
mod tests {
    use diesel::{QueryDsl, RunQueryDsl};
    use uuid::Uuid;
    use crate::lib::database;
    use crate::lib::database::inventory::upsert::upsert_inventory;
    use crate::lib::database::models::Inventory;
    use tokio_test::block_on;
    use crate::lib::database::schema::inventory;

    macro_rules! aw {
    ($e:expr) => {
        block_on($e)
    };
  }
    #[test]
    fn test_upsert() {
        let mut conn = database::establish_connection();

        let mut new_inventory = Inventory::default();
        let sample_id = Uuid::new_v4().to_string();
        let sample_vin = "WBAAB5409J8871789".to_string();
        new_inventory.id = sample_id.clone();
        new_inventory.vin = sample_vin.clone();

        let sanitized = new_inventory.sanitize();

        //     first, insert
        let result = aw!(upsert_inventory(sanitized.clone()));

        let is_ok = result.is_ok();
        if (!is_ok) {
            println!("Could not insert: {:?}", result.unwrap_err());
        }

        assert_eq!(is_ok, true);

        // then, upsert
        let test_make = "FORD".to_string();
        new_inventory.make = test_make.clone();
        let sanitized = new_inventory.sanitize();
        let result = aw!(upsert_inventory(sanitized.clone()));

        assert_eq!(result.is_ok(), true);

        let result = result.unwrap();

        assert_eq!(result.make, test_make);
        assert_eq!(result.vin, sample_vin);


        // finally, delete
        let result = diesel::delete(inventory::table.find(new_inventory.id)).execute(&mut conn);

        assert_eq!(result.is_ok(), true);

    }
}