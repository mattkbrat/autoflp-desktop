use diesel::{QueryDsl, RunQueryDsl};
use crate::lib::database;
use crate::lib::database::schema::{account, person};
use crate::lib::database::schema::person::{first_name, id as personId, last_name};


pub type AccountPeople = Vec<(String, String, String)>;

pub fn get_account_people() -> AccountPeople {
    let mut conn = database::establish_connection();

    let all_names = person::table
        .inner_join(account::table)
        .select((last_name, first_name, personId))
        .distinct()
        .order((last_name, first_name))
        .load(&mut conn);

    all_names.unwrap()
}
