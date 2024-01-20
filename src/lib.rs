pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{Person};
use crate::schema::{person};
use crate::schema::charge::name;
use crate::schema::person::{first_name, last_name};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_account() -> Person {
    let mut conn = establish_connection();


    let momo = person::table
        .filter(person::first_name.eq("matthew"))
        .select(Person::as_select())
        .get_result(&mut conn);

    momo.unwrap()
}

pub fn get_people() -> Vec<(String, String)> {
    let mut conn = establish_connection();

    let all_names = person::table.select((last_name, first_name))
        .order((last_name, first_name))
        .load(&mut conn);

    all_names.unwrap()

}