use crate::lib::database::account::get_account::get_account;
use crate::lib::database::deal::DealsByAccount;
use crate::lib::database::{deal, establish_connection, models};
use deal::get_deals_by_account;
use get_deals_by_account::get_deals_by_account;
use models::{Account, Person};

pub fn get_account_details(
    account_id: Option<String>,
) -> Option<(Person, Option<Account>, DealsByAccount)> {
    let mut conn = establish_connection();

    account_id.as_ref()?;

    let this_id = account_id.unwrap();

    let account = get_account(Some(&this_id), &mut conn);

    account.as_ref()?;

    let account = account.unwrap();

    let (person, account) = account;

    let mut deals = Vec::new();

    if let Some(account) = account.clone() {
        println!("got id {}", account.id);

        deals = get_deals_by_account(&account.id, &mut conn);
    }

    Some((person, account, deals))
}
