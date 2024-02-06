use chrono::{Datelike, NaiveDate, Utc};
use chronoutil::RelativeDuration;

use crate::lib::finance::DateLogic;

pub fn get_days_from_month(year: i32, month: u32) -> Option<u32> {
    let since = NaiveDate::from_ymd_opt(year, month, 1);


    let days: i32 = if let Some(since) = since {
        let date1 = NaiveDate::from_ymd_opt(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        );

        if let Some(date1) = date1 {
            date1.signed_duration_since(since).num_days() as i32
        } else {
            -1
        }
    } else {
        -1
    };

    if days > 0 {
        Some(days as u32)
    } else {
        None
    }
}

pub fn add_months(start: NaiveDate, months: i32) -> NaiveDate {
    let relative_months = RelativeDuration::months(months);
    start + relative_months
}


pub fn months_between_dates(start: &NaiveDate, end: &NaiveDate) -> i32 {
    let month_diff = end.month() as i32 - start.month() as i32;
    let years_diff = end.year() - start.year();

    if month_diff >= 0 {
        (years_diff * 12) + month_diff
    } else {
        (years_diff - 1) * 12 + (month_diff + 12)
    }
}

pub fn get_next_payment_date(params: DateLogic) -> NaiveDate {

    // Start the next payment at the date of the deal + months since the deal.
    let mut next_payment = add_months(params.deal_date, months_between_dates(&params.deal_date, &params.current_date));

    // If the account is not paid in advance (delinquent or standard),
    // then the nex payment must be this month or the next month if the payment date has passed.
    if params.months_delinquent >= 0 {
        if next_payment > Utc::now().date_naive() {
            return add_months(next_payment, 1);
        }
        return next_payment;
    }

    add_months(next_payment, params.months_delinquent.abs())
}
