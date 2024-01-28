pub mod models;
pub mod schema;
pub mod finance;

use crate::models::{Account, Person};
use crate::schema::account::contact;
use crate::schema::person::{first_name, id, last_name};
use crate::schema::{account, person};
use diesel::prelude::*;
use diesel::query_dsl::InternalJoinDsl;
use dotenvy::dotenv;
use either::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_account(account_id: Option<String>) -> Option<(Person, Account)> {
    if account_id.is_none() {
        return None;
    };

    let this_id = account_id.unwrap();

    println!("{this_id}");

    if (this_id.len() <= 0) {
        return None;
    }

    let mut conn = establish_connection();

    let selected_person = account::table
        .inner_join(person::table)
        .filter(id.eq(this_id))
        .select((Person::as_select(), Account::as_select()))
        .load::<(Person, Account)>(&mut conn);

    selected_person.unwrap().into_iter().nth(0)
}

pub fn get_people() -> Vec<(String, String, String)> {
    let mut conn = establish_connection();

    let all_names = person::table
        .select((last_name, first_name, id))
        .distinct()
        .order((last_name, first_name))
        .load(&mut conn);

    all_names.unwrap()
}

