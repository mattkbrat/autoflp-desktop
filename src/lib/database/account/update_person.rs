use diesel::RunQueryDsl;
use crate::lib::database;
use crate::lib::database::models::{Person, PersonForm};


pub async fn update_details(person: PersonForm) -> Result<i32, String> {
    let mut conn = database::establish_connection();

    let updated = diesel::update(&person)
        .set(person.clone())
        .execute(&mut conn);

    if updated.is_ok() {
        let updated = updated.unwrap();
        println!("{:?} account updated", updated );
        // TODO: Send notification
        return Ok(200);
    }

    let error_message = "Could not update: ".to_owned() + &updated.unwrap_err().to_string();

    return Err(error_message);
}

