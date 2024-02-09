use chrono;
use chrono::{NaiveDate, Local};


pub fn get_today() -> NaiveDate {
    Local::now().date_naive()
}