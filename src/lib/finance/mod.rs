use chrono::NaiveDate;

pub struct DateLogic {
    pub deal_date: NaiveDate,
    pub has_paid_this_month: bool,
    pub months_delinquent: i32,
    pub current_date: NaiveDate,
}

pub mod add;
mod amortization_schedule;
pub(crate) mod calc;
pub mod next_payment_date;
pub mod round_to_penny;
