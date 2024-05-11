use crate::lib::database;
use crate::lib::database::schema::person::{first_name, id as personId, last_name};
use crate::lib::database::schema::{account, person};
use diesel::{QueryDsl, RunQueryDsl};

pub type AccountPeople = Vec<(String, String, String)>;

pub fn get_account_people() -> AccountPeople {
    let mut conn = database::establish_connection();

    let all_names = person::table
        .select((last_name, first_name, personId))
        .distinct()
        .order((last_name, first_name))
        .load(&mut conn);

    all_names.unwrap()
}
