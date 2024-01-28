use chrono::NaiveDate;

pub struct DateLogic {
    deal_date: NaiveDate,
    has_paid_this_month: bool,
    months_delinquent: i32,
    current_date: NaiveDate,
}


pub mod add;
pub mod round_to_penny;
pub mod next_payment_date;
pub mod next_payment_date_test;
