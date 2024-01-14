pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{Person};
use crate::schema::{person};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_account() -> Person {
    use crate::schema::account;

    let mut conn = establish_connection();

    // let account = Account {
    //     id: "".to_string(),
    //     contact: "".to_string(),
    //     cosigner: None,
    //     date_of_birth: None,
    //     license_number: "".to_string(),
    //     license_expiration: None,
    //     date_added: None,
    //     date_modified: None,
    //     current_standing: None,
    //     notes: None,
    // };

    let momo = person::table
        .filter(person::first_name.eq("matthew"))
        .select(Person::as_select())
        .get_result(&mut conn);

    momo.unwrap()
}

// pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
//     use crate::schema::posts;
//
//     let new_post = NewPost { title, body };
//
//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .returning(Post::as_returning())
//         .get_result(conn)
//         .expect("Error saving new post")
// }