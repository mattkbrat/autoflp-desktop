use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub mod account;
pub mod models;
pub mod schema;
pub mod payment;
pub mod deal;
 pub mod inventory;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
