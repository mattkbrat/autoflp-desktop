use crate::lib::finance::calc::FinanceCalc;
use chrono::{Datelike, NaiveDate};
use chronoutil::RelativeDuration;

use crate::lib::finance::next_payment_date::months_between_dates;

#[derive(Debug, Clone)]
struct FinanceHistoryRow<'r> {
    date: &'r str,
    owed: &'r str,
    paid: &'r str,
    balance: &'r str,
    delinquent: &'r str,
}

type FinanceHistory<'r> = Vec<FinanceHistoryRow<'r>>;

struct AmortizationScheduleParams<'r> {
    principal: &'r f64,
    annual_rate: &'r f32,
    number_of_payments: &'r i8,
    pmt: &'r f64,
    history: Option<FinanceHistory<'r>>,
    finance: FinanceCalc,
}

type SchedulePayment = (f64, NaiveDate);

struct ScheduleRow {
    date: NaiveDate,
    payment: Vec<SchedulePayment>, // Allow for multiple payments in one month.
    total_paid: f64,
    beginning_balance: f64,
    interest: f64,
    principal: f64,
    balance: f64,
    index: i8,
    expected_payment: f64,
}

type Schedule = Vec<ScheduleRow>;

struct AmortizationSchedule {
    schedule: Schedule,
    total_paid: f32,
    total_interest: f32,
    total_principal: f32,
}

pub(crate) fn get_amortization_schedule(p: AmortizationScheduleParams) -> AmortizationSchedule {
    let finance = p.finance;
    let history = p.history;
    let monthly_rate = match p.annual_rate > &0.0 {
        true => p.annual_rate / 100.0,
        false => 1.0,
    } / 12.0;

    let start_date =
        NaiveDate::parse_from_str(&finance.first_payment_due_date, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(&finance.last_payment_due_date, "%Y-%m-%d").unwrap();

    let mut total_paid = f64::from(0.0);
    let mut total_interest = f64::from(0.0);
    let mut total_principal = f64::from(0.0);

    let mut balance = finance.finance_amount.clone();

    let (number_of_payments, row_count, history) = match history {
        Some(x) => (
            x.len(),
            months_between_dates(&start_date, &end_date),
            x.clone(),
        ),
        None => (
            *p.number_of_payments as usize,
            *p.number_of_payments as i32,
            Vec::new(),
        ),
    };

    let mut temp_schedule: Schedule = Vec::new();

    for i in 0..row_count {
        let relative_months = RelativeDuration::months(i);
        let date = start_date + relative_months;
        let interest = balance * monthly_rate as f64;
        let pmt = p.pmt.clone();
        let principal = pmt as f64 - interest;
        balance -= principal;

        total_paid += p.pmt;
        total_interest += interest;
        total_principal += principal;

        let mut payment_vec: Vec<SchedulePayment> = Vec::new();

        // Only insert the default payment if there are no payments in the history.
        // (New schedule)
        if history.len() == 0 {
            payment_vec.push((pmt, date));
        }

        temp_schedule.push(ScheduleRow {
            date,
            payment: payment_vec,
            total_paid: 0.0,
            beginning_balance: (balance + principal),
            interest,
            principal,
            balance,
            index: i as i8,
            expected_payment: pmt,
        });
    }

    let mut payments_before_start = f64::from(0.0);

    if history.len() > 0 {
        for h in 0..history.len() {
            let history = &history[h];
            let date_check = history.date;
            let date_check_parsed = NaiveDate::parse_from_str(date_check, "%Y-%m-%d");
            if date_check_parsed.is_err() {
                println!("Error parsing date {}", date_check);
                continue;
            }
            let date_check = date_check_parsed.unwrap();
            let date_check_month = date_check.month();
            let date_check_year = date_check.year();

            let paid = history
                .paid
                .parse::<f64>()
                .expect("Error parsing history.paid");
            let date = history
                .date
                .parse::<NaiveDate>()
                .expect("Error parsing history.date");

            if date_check < start_date {
                payments_before_start += paid.clone();
                continue;
            }

            let months_between = months_between_dates(&start_date, &date_check);

            let schedule_index = temp_schedule.iter().position(|x| {
                let month = x.date.month();
                let year = x.date.year();
                month == date_check_month && year == date_check_year
            });

            if schedule_index.is_none() {
                println!(
                    "Payment with date {} is outside of the schedule",
                    date_check
                );
                continue;
            }

            let schedule_index = schedule_index.unwrap();

            // Allow for multiple payments in one month.
            temp_schedule[schedule_index]
                .payment
                .push((paid.into(), date));
            // let total_paid_month = temp_schedule[schedule_index].payment.iter().map(|x| x.0).sum::<f32>();
            // let expected_paid_month = temp_schedule[schedule_index.unwrap()].expected_payment;
            temp_schedule[schedule_index].total_paid += paid;

            total_paid += paid;
        }
    }

    AmortizationSchedule {
        schedule: temp_schedule,
        total_paid: total_paid as f32,
        total_interest: total_interest as f32,
        total_principal: total_principal as f32,
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::finance::calc::FinanceCalc;

    #[test]
    fn sample_test_for_compilation() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_amortization_schedule() {
        use crate::lib::finance::amortization_schedule::{
            get_amortization_schedule, AmortizationSchedule, AmortizationScheduleParams,
            FinanceCalc, FinanceHistoryRow,
        };
        use chrono::NaiveDate;
        let params = AmortizationScheduleParams {
            principal: &10000.0,
            annual_rate: &5.0,
            number_of_payments: &12,
            pmt: &1000.0,
            history: Some(vec![
                FinanceHistoryRow {
                    date: "2020-01-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "9000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-02-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "8000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-03-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "7000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-04-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "6000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-05-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "5000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-06-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "4000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-07-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "3000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-08-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "2000",
                    delinquent: "0",
                },
                FinanceHistoryRow {
                    date: "2020-09-01",
                    owed: "1000",
                    paid: "1000",
                    balance: "1000",
                    delinquent: "0",
                },
            ]),
            finance: FinanceCalc {
                selling_trade_differential: 0.0,
                state_tax_dollar: 0.0,
                county_tax_dollar: 0.0,
                city_tax_dollar: 0.0,
                rtd_tax_dollar: 0.0,
                total_tax_dollar: 0.0,
                total_tax_percent: 0.0,
                cash_balance_with_tax: 0.0,
                unpaid_cash_balance: 0.0,
                finance_amount: 0.0,
                total_loan: 0.0,
                deferred_payment: 0.0,
                monthly_payment: 0.0,
                last_payment: 0.0,
                last_payment_due_date: "2020-12-01".to_string(),
                first_payment_due_date: "2020-01-01".to_string(),
                deferred: 0.0,
                total_cost: 0.0,
            },
        };
        let schedule = get_amortization_schedule(params);
        assert_eq!(schedule.total_paid, 9000.0);
        assert_eq!(schedule.total_interest, 0.0);
        assert_eq!(schedule.total_principal, 9000.0);
    }
}

