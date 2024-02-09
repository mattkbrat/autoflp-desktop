use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use crate::lib::database::deal::DealDetails;
use crate::lib::database::models::{Creditor, Deal, Inventory};
use crate::lib::database::schema::{account, creditor, deal, inventory};

pub fn get_deal_by_id(deal_id: &String, conn: &mut SqliteConnection) -> Option<DealDetails> {
    let selected_deal = deal::table
        .inner_join(inventory::table.on(inventory::vin.eq(deal::inventory)))
        .inner_join(account::table)
        .left_outer_join(creditor::table)
        .select((
            Deal::as_select(),
            Inventory::as_select(),
            Option::<Creditor>::as_select(),
        ))
        .filter(deal::id.eq(deal_id))
        // .first::<DealDetails>(conn)
        // .order_by(deal::date.desc())
        .load::<DealDetails>(conn)
        .unwrap();

    selected_deal.into_iter().nth(0)
}
