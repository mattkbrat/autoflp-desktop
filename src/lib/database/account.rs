
use crate::lib::database::models::{Account, Creditor, Deal, Inventory, Payment, Person};
use crate::lib::database::schema::deal::account as dealAccount;
use crate::lib::database::schema::person::{first_name, id as personId, last_name};
use crate::lib::database::schema::{account, creditor, deal, inventory, person};

use diesel::prelude::*;
use crate::lib::database;

pub fn get_account(person_id: Option<&String>) -> Option<(Person, Account)> {
    if person_id.is_none() {
        return None;
    };

    let this_id = person_id.unwrap();

    if this_id.len() <= 0 {
        return None;
    }

    let mut conn = database::establish_connection();

    let selected_person = account::table
        .inner_join(person::table)
        .filter(personId.eq(this_id))
        .select((Person::as_select(), Account::as_select()))
        .load::<(Person, Account)>(&mut conn);

    selected_person.unwrap().into_iter().nth(0)
}

pub fn get_people() -> Vec<(String, String, String)> {
    let mut conn = database::establish_connection();

    let all_names = person::table
        .select((last_name, first_name, personId))
        .distinct()
        .order((last_name, first_name))
        .load(&mut conn);

    all_names.unwrap()
}

pub fn get_account_people() -> Vec<(String, String, String)> {
    let mut conn = database::establish_connection();

    let all_names = person::table
        .inner_join(account::table)
        .select((last_name, first_name, personId))
        .distinct()
        .order((last_name, first_name))
        .load(&mut conn);

    all_names.unwrap()
}

pub fn get_account_deals(account_id: Option<&String>) -> Option<Vec<(Deal, Inventory)>> {
    let mut conn = database::establish_connection();
    if account_id.is_none() {
        println!("No account id");
        return None;
    };

    let this_id = account_id.unwrap();

    println!("Account id: {}", this_id);

    let account_deals = deal::table
        .inner_join(inventory::table)
        .filter(dealAccount.eq(this_id))
        .select((Deal::as_select(), Inventory::as_select()))
        .load::<(Deal, Inventory)>(&mut conn)
        .unwrap();

    Some(account_deals)
}

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
    Option<Vec<((Deal, Inventory, Option<Creditor>), Vec<Payment>)>>,
    Person,
    Account,
)>;

pub fn get_account_details(person_id: Option<String>) -> AccountDetails {
    if person_id.is_none() {
        return None;
    };

    let this_id = person_id.unwrap();

    let mut conn = database::establish_connection();

    let account = get_account(Some(&this_id));

    if account.is_none() {
        println!("No account");
        return None;
    }

    let account = account.unwrap();

    let (person, account) = account;

    // let account_details= deal::table
    //     .inner_join(account::table)
    //     .inner_join(inventory::table)
    //     .filter(dealAccount.eq(account.id.clone()))
    //     .load::<(Deal, Inventory)>(&mut conn);

    // First, select the matching deals by account Id.
    // Then, select the inventory belonging to the deals through the vin.
    let selected_deal = deal::table
        .inner_join(inventory::table.on(inventory::vin.eq(deal::inventory)))
        .inner_join(account::table)
        .left_outer_join(creditor::table)
        .filter(dealAccount.eq(account.id.clone()))
        .select((
            Deal::as_select(),
            Inventory::as_select(),
            Option::<Creditor>::as_select(),
        ))
        .load::<(Deal, Inventory, Option<Creditor>)>(&mut conn)
        .unwrap();

    // if selected_deal.len() <= 0 {
    //     println!("No deals {}", account.id);
    //     return Some((None, person, account));
    // }



    let deals: Vec<&Deal> = selected_deal.iter().map(|d| &d.0).collect();

    let payments = Payment::belonging_to(&deals)
        .select(Payment::as_select())
        .load(&mut conn)
        .unwrap()
        ;

    let payments_by_deal: Vec<((Deal, Inventory, Option<Creditor>), Vec<Payment>)> = payments.grouped_by(&deals)
        .into_iter()
        .zip( selected_deal.into_iter())
        .map(|(p, d)| (d, p))
        .collect()
        ;

    // let account_deals = deal::table
    //     .filter(dealAccount.eq(account.id.clone()))
    //     .select(Deal::as_select())
    //     .load(&mut conn)
    //     .unwrap()
    //     ;
    //
    // let deals_iter = account_deals.into_iter();
    //
    // let inventory_in_deals = inventory::table.filter(vin.eq((deals_iter.map(|d| d.inventory))))
    //     .select(Inventory::as_select())
    //     .load(&mut conn)
    //     .unwrap()
    //     ;



    Some((Some(payments_by_deal), person, account))
}

#[cfg(test)]
mod tests {
    use crate::lib::database::{establish_connection, account};
    use account::get_account_details;

    #[test]
    fn test_deal_account_inventory() {
        let mut conn = establish_connection();
        let person_id = "2c4d4cc8-5729-44bb-ad71-ec79dd7fedef";

        let result = get_account_details(Some(person_id.to_string()));

        assert_eq!(1, 1);
    }
}

fn main() {}