use crate::lib::database;
use crate::lib::database::deal::PaymentsByDeal;
use crate::lib::database::deal::{get_deal, get_deal_payments};

pub fn get_deal_details(deal_id: Option<String>) -> Option<PaymentsByDeal> {
    if deal_id.is_none() {
        return None;
    };

    let this_id = deal_id.unwrap();

    let mut conn = database::establish_connection();

    let selected_deal = get_deal::get_deal_by_id(&this_id, &mut conn);

    if selected_deal.is_none() {
        return None
    }

    let (deal, inventory, creditor) = selected_deal.unwrap();

    // let deal: &Deal = selected_deal.iter().map(|d| &d.0).collect();
    let payments = get_deal_payments::get_deal_payments(&deal, &mut conn);

    // let payments_by_deal: PaymentsByDeal = payments.grouped_by(&deals)
    //     .into_iter()
    //     .zip( selected_deal.into_iter())
    //     .map(|(p, d)| (d, p))
    //     .collect()
    //     ;

    Some((deal, inventory, creditor, payments))
}
