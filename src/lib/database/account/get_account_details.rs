use crate::lib::database::{establish_connection, models, deal};
use models::{Account, Person};
use deal::{get_deals_by_account};
use get_deals_by_account::{get_deals_by_account};
use crate::lib::database::account::get_account::get_account;
use crate::lib::database::deal::DealsByAccount;

pub fn get_account_details(account_id: Option<String>) -> Option<(Person, Account, DealsByAccount)> {
    let mut conn = establish_connection();

    if account_id.is_none() {
        return None;
    };

    let this_id = account_id.unwrap();

    let account = get_account(Some(&this_id), &mut conn);

    if account.is_none() {
        return None;
    }


    let account = account.unwrap();


    let (person, account) = account;

    let deals = get_deals_by_account(&account.id, &mut conn);


    Some((person, account, deals))
}
