// {
//   tax: { city, county, rtd, state },
//   prices: { selling, down, trade },
//   creditor: { filingFees, apr, term },
//   firstPayment,
// }: {
//   tax: {
//     city: number;
//     county: number;
//     rtd: number;
//     state: number;
//   };
//   prices: {
//     selling: number;
//     down: number;
//     trade: number;
//   };
//   creditor: {
//     filingFees: number;
//     apr: number;
//     term: number;
//   };
//   firstPayment: Date;
// }

use crate::lib::database::schema::deal::{down, term};
use crate::lib::database::schema::person::city;
use chrono::NaiveDate;
use crate::lib::finance::next_payment_date::add_months;

pub(crate) struct Taxes<'r> {
    pub(crate) city: &'r f64,
    pub(crate) county: &'r f64,
    pub(crate) rtd: &'r f64,
    pub(crate) state: &'r f64,
}

pub(crate) struct Prices<'r> {
    pub(crate) selling: &'r f64,
    pub(crate) down: &'r f64,
    pub(crate) trade: &'r f64,
}

pub(crate) struct Creditor<'r> {
    pub(crate) filingFees: &'r f64,
    pub(crate) apr: &'r f64,
    pub(crate) term: &'r i32,
}

pub(crate) struct FinanceCalcParams<'r> {
    pub(crate) tax: &'r Taxes<'r>,
    pub(crate) prices: &'r Prices<'r>,
    pub(crate) creditor: &'r Creditor<'r>,
    pub(crate) first_payment: &'r NaiveDate,
}

pub(crate) struct FinanceCalc<'r> {
    pub(crate) selling_trade_differential: &'r f64,
    pub(crate) state_tax_dollar: &'r f64,
    pub(crate) county_tax_dollar: &'r f64,
    pub(crate) city_tax_dollar: &'r f64,
    pub(crate) rtd_tax_dollar: &'r f64,
    pub(crate) total_tax_dollar: &'r f64,
    pub(crate) total_tax_percent: &'r f64,
    pub(crate) cash_balance_with_tax: &'r f64,
    pub(crate) unpaid_cash_balance: &'r f64,
    pub(crate) finance_amount: &'r f64,
    pub(crate) total_loan: &'r f64,
    pub(crate) deferred_payment: &'r f64,
    pub(crate) monthly_payment: &'r f64,
    pub(crate) last_payment: &'r f64,
    pub(crate) last_payment_due_date: &'r str,
    pub(crate) first_payment_due_date: &'r str,
    pub(crate) deferred: &'r f64,
    pub(crate) total_cost: &'r f64,
}

fn get_percent(num: &f64) -> f64 {
    num / match num > &0.0 {
        true => 100.0,
        false => 1.0,
    }
}

pub(crate) fn calculate_finance(p: FinanceCalcParams) -> FinanceCalc {
    let tax = p.tax;
    let prices = p.prices;
    let creditor = p.creditor;
    let first_payment = p.first_payment;

    // Normalize the tax rates to a percentage
    let city_tax = get_percent(tax.city);
    let county_tax = get_percent(tax.county);
    let rtd_tax = get_percent(tax.rtd);
    let state_tax = get_percent(tax.state);

    let selling_trade_diff = prices.selling - prices.trade;

    let total_tax_percent = city_tax + county_tax + rtd_tax + state_tax;
    let total_tax_dollar = selling_trade_diff * (total_tax_percent / 100.0);

    // Q: Should down be included in the selling_trade_diff?
    // A: No, down is a separate amount that is paid at the time of the deal.
    let unpaid_cash_balance = selling_trade_diff - prices.down - total_tax_dollar;

    let finance_amount = unpaid_cash_balance + creditor.filingFees;

    let cash_balance_with_tax = unpaid_cash_balance + total_tax_dollar;
    let cash_balance_with_tax_and_down = cash_balance_with_tax + prices.down; // For a form field
    let deferred = cash_balance_with_tax_and_down - finance_amount;

    // If we wanted the down payment to be included in the finance amount, we would use this:
    // let finance_amount = selling_trade_diff - total_tax_dollar + creditor.filingFees;

    // If we wanted the down payment to be 0,
    // let finance_amount = selling_trade_diff - prices.down - total_tax_dollar + creditor.filingFees;

    let mut finance_calc = FinanceCalc {
        selling_trade_differential: &selling_trade_diff,
        state_tax_dollar: &(selling_trade_diff * (state_tax / 100.0)),
        county_tax_dollar: &(selling_trade_diff * (county_tax / 100.0)),
        city_tax_dollar: &(selling_trade_diff * (city_tax / 100.0)),
        rtd_tax_dollar: &(selling_trade_diff * (rtd_tax / 100.0)),
        total_tax_dollar: &total_tax_dollar,
        total_tax_percent: &total_tax_percent,
        cash_balance_with_tax: &cash_balance_with_tax,
        unpaid_cash_balance: &unpaid_cash_balance,
        finance_amount: &finance_amount,
        total_loan: &(finance_amount + deferred),
        deferred_payment: &deferred,
        monthly_payment: &(finance_amount / f64::from( creditor.term.clone())),
        last_payment: &(finance_amount + deferred),
        last_payment_due_date: &first_payment.to_string(),
        first_payment_due_date: &first_payment.to_string(),
        deferred: &deferred,
        total_cost: &(selling_trade_diff + total_tax_dollar + creditor.filingFees),
    };


    if *creditor.term <= 0 {
        return finance_calc;
    }

    // Calculate the monthly payment
    let interest_rate = creditor.apr / 12.0 / 100.0;
    let term_float = f64::from(*creditor.term);

    let payment = (finance_amount * interest_rate * (1.0 + interest_rate).powf(term_float))
        / ((1.0 + interest_rate).powf(term_float) - 1.0);

    let total_loan_amount = payment * term_float;

    let deferred_payment = total_loan_amount - finance_amount;

    let payment_rounded_to_cents = (payment * 100.0).round() / 100.0;

    // The last payment should account for the difference in cents.
    let last_payment = payment_rounded_to_cents + total_loan_amount - payment_rounded_to_cents * term_float;

    let last_payment_due_date = add_months(*first_payment, *creditor.term);

    let total_cost = prices.selling + total_tax_dollar + creditor.filingFees + deferred_payment;

    finance_calc.monthly_payment = &payment_rounded_to_cents;
    finance_calc.last_payment = &last_payment;
    finance_calc.last_payment_due_date = &last_payment_due_date.to_string();
    finance_calc.total_loan = &total_loan_amount;
    finance_calc.deferred_payment = &deferred_payment;
    finance_calc.total_cost = &total_cost;

    finance_calc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_finance() {
        let tax = Taxes {
            city: &0.0,
            county: &0.0,
            rtd: &0.0,
            state: &0.0,
        };

        let prices = Prices {
            selling: &10000.0,
            down: &0.0,
            trade: &0.0,
        };

        let creditor = Creditor {
            filingFees: &0.0,
            apr: &0.0,
            term: &0,
        };

        let first_payment = NaiveDate::from_ymd(2020, 1, 1);

        let params = FinanceCalcParams {
            tax: &tax,
            prices: &prices,
            creditor: &creditor,
            first_payment: &first_payment,
        };

        let finance = calculate_finance(params);

        assert_eq!(finance.selling_trade_differential, &10000.0);
        assert_eq!(finance.state_tax_dollar, &0.0);
        assert_eq!(finance.county_tax_dollar, &0.0);
        assert_eq!(finance.city_tax_dollar, &0.0);
        assert_eq!(finance.rtd_tax_dollar, &0.0);
        assert_eq!(finance.total_tax_dollar, &0.0);
        assert_eq!(finance.total_tax_percent, &0.0);
        assert_eq!(finance.cash_balance_with_tax, &0.0);
        assert_eq!(finance.unpaid_cash_balance, &10000.0);
        assert_eq!(finance.finance_amount, &0.0);
        assert_eq!(finance.total_loan, &0.0);
        assert_eq!(finance.deferred_payment, &0.0);
        assert_eq!(finance.monthly_payment, &0.0);
        assert_eq!(finance.last_payment, &0.0);
        assert_eq!(finance.last_payment_due_date, "2020-01-01");
        assert_eq!(finance.first_payment_due_date, "2020-01-01");
        assert_eq!(finance.deferred, &0.0);
        assert_eq!(finance.total_cost, &10000.0);
    }
}
