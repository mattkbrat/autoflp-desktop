use crate::lib::database::models::{self, PersonForm};
use crate::lib::database::{self, schema};
use diesel::RunQueryDsl;
use uuid::Uuid;

pub async fn update_details(mut person: PersonForm) -> Result<i32, String> {
    let mut conn = database::establish_connection();
    let mut id = person.id.clone();
    let is_new_account = id.is_empty();

    let mut result = 0;
    if is_new_account {
        println!("new account");
        id = Uuid::new_v4().to_string();
        person.id = id;

        let inserted = diesel::insert_into(schema::person::table)
            .values(&person)
            .execute(&mut conn);

        if inserted.is_err() {
            let error_message = format!("Could not insert: {:?}", inserted.unwrap_err());

            return Err(error_message);
        }
        result = inserted.unwrap();
    } else {
        print!("updating {:?}", person);
        let updated = diesel::update(&person)
            .set(person.clone())
            .execute(&mut conn);

        if updated.is_err() {
            let error_message = "Could not update: ".to_owned() + &updated.unwrap_err().to_string();

            return Err(error_message);
        }
        result = updated.unwrap();
    }

    println!("{:?} account success", result);
    // TODO: Send notification
    Ok(match is_new_account {
        true => 201,
        false => 200,
    })
}
