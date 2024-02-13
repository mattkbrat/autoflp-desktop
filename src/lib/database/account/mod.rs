use crate::lib::database::models::{Account, Deal, Payment, Person};
use crate::lib::database::schema::payment;

use diesel::prelude::*;
use crate::lib::database::deal::AccountDeal;

pub mod get_account;
pub mod get_people;
pub mod get_account_people;
pub mod get_account_details;
pub mod update_person;

// pub fn get_account_details(account_id: Option<String>) -> Option<(Vec<(Deal, Inventory)>, Person, Account)> {
//     if account_id.is_none() {
//         return None;
//     };
//
//     let this_id = account_id.unwrap();
//
//     if this_id.len() <= 0 {
//         return None;
//     }
//
//     let account = get_account(Some(&this_id));
//
//     if account.is_none() {
//         println!("No account");
//         return None;
//     }
//
//     let account = account.unwrap();
//
//     let (person, account) = account;
//
//     let deals = get_account_deals(Some(&account.id));
//
//     let deals = deals.unwrap();
//
//     if deals.len() <= 0{
//         println!("No deals {} {}", account.id, deals.len());
//         return None;
//     }
//
//     Some((deals, person, account))
// }

// select lien, deal.state, i.make, i.model from deal
//     join main.account a on a.id = deal.account
//     join inventory i on deal.inventory = i.vin
// where a.id = '2dece2ee-e8b9-4560-8b57-13e4d8f1b5d5'




pub type AccountDetails = Option<(
    Option<Vec<(AccountDeal, Vec<Payment>)>>,
    Person,
    Account,
)>;

fn _get_deals_payments(deals: Vec<&Deal>, conn: &mut SqliteConnection) -> Vec<Payment> {

    let payments = Payment::belonging_to(&deals)
        .select(Payment::as_select())
        .order_by(payment::date.desc())
        .load(conn)
        .unwrap()
        ;

    payments
}
