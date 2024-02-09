use diesel::{QueryDsl, RunQueryDsl};
use crate::lib::database;
use crate::lib::database::schema::person;
use crate::lib::database::schema::person::{first_name, id as personId, last_name};

pub fn get_people() -> Vec<(String, String, String)> {
    let mut conn = database::establish_connection();

    let all_names = person::table
        .select((last_name, first_name, personId))
        .distinct()
        .order((last_name, first_name))
        .load(&mut conn);

    all_names.unwrap()
}
