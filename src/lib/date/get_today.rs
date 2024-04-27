use chrono;
use chrono::{NaiveDate, Local};
use chrono::format::{DelayedFormat, StrftimeItems};


pub fn get_today() -> NaiveDate {
    Local::now().date_naive()
}
pub fn get_today_string<'a>() -> DelayedFormat<StrftimeItems<'a>> {
    let today = chrono::offset::Local::now().format("%Y-%m-%d");
    today
}