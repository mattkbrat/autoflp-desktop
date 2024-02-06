#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::lib::finance::DateLogic;
    use crate::lib::finance::next_payment_date::{add_months, get_next_payment_date, months_between_dates};

    #[test]
    fn test_date_plus_months() {
        let current_year = 2024;

        let date = NaiveDate::from_ymd_opt(current_year, 1, 1);
        if let Some(date) = date {
            let new_date = add_months(date, 4);
            assert_eq!(Some(new_date), NaiveDate::from_ymd_opt(current_year, 5, 1));
        } else {
            panic!("Date is not valid");
        }
    }

    #[test]
    fn test_date_plus_months_increases_year() {
        let current_year = 2024;
        let plus_months = 4;

        let date = NaiveDate::from_ymd_opt(current_year, 11, 1);
        let expected_date = NaiveDate::from_ymd_opt(current_year + 1, 3, 1);
        if let Some(date) = date {
            let new_date = add_months(date, plus_months);
            assert_eq!(Some(new_date), expected_date);
        } else {
            panic!("Date is not valid");
        }
    }

    #[test]
    fn test_months_between_dates() {
        let current_year = 2024;

        let date = NaiveDate::from_ymd_opt(current_year, 1, 1);
        if let Some(date) = date {
            let new_date = add_months(date, 4);
            assert_eq!(months_between_dates(&date, &new_date), 4);
        } else {
            panic!("Date is not valid");
        }
    }

    #[test]
    fn test_months_between_dates_with_negative() {
        let current_year = 2024;

        let date = NaiveDate::from_ymd_opt(current_year, 1, 1);
        if let Some(date) = date {
            let new_date = add_months(date, -4);
            assert_eq!(months_between_dates(&date, &new_date), -4);
        } else {
            panic!("Date is not valid");
        }
    }

    #[test]
    fn test_get_next_payment_date_paid_in_advance() {
        // Test that if the account is paid in advance, the next payment date is x months plus today.

        let deal_date = NaiveDate::from_ymd_opt(2020, 1, 1);
        let current_date = NaiveDate::from_ymd_opt(2020, 1, 1);

        if let Some(deal_date) = deal_date {
            if let Some(current_date) = current_date {
                let params = DateLogic {
                    deal_date: deal_date,
                    has_paid_this_month: true,
                    months_delinquent: -2,
                    current_date: current_date,
                };

                let next_payment_date = get_next_payment_date(params);
                let expected_date = NaiveDate::from_ymd_opt(2020, 3, 1);
                assert_eq!(Some(next_payment_date), expected_date);
            } else {
                panic!("Date is not valid");
            }
        } else {
            panic!("Date is not valid");
        }
    }
    //
    #[test]
    fn test_get_next_payment_date_not_delinquent() {
        let params = DateLogic {
            deal_date: NaiveDate::from_ymd(2020, 1, 1),
            has_paid_this_month: false,
            months_delinquent: 0,
            current_date: NaiveDate::from_ymd(2019, 11, 10),
        };

        let next_payment_date = get_next_payment_date(params);
        let expected_date = NaiveDate::from_ymd_opt(2019, 11, 01);
        assert_eq!(Some(next_payment_date), expected_date);
    }

    #[test]
    fn test_get_next_payment_date_delinquent_and_date_after() {
        // Test that if the account is not paid in advance, the next payment date is the upcoming month
        // plus the original payment date (1/1/2020) plus the number of months since the deal date (1/1/2020)
        // and that the account has paid this month (2/1/2020)
        let params = DateLogic {
            deal_date: NaiveDate::from_ymd(2020, 1, 1),
            has_paid_this_month: true,
            months_delinquent: 4,
            current_date: NaiveDate::from_ymd(2022, 11, 1),
        };

        let next_payment_date = get_next_payment_date(params);
        let expected_date = NaiveDate::from_ymd_opt(2022, 11, 1);
        assert_eq!(Some(next_payment_date), expected_date);
    }
}
