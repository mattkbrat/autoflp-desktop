use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel::query_dsl::InternalJoinDsl;
use crate::lib::database::deal::{DealByAccount, DealsByAccount};
use crate::lib::database::models::{Account, Deal};
use crate::lib::database::schema::{deal, inventory};


pub fn get_deals_by_account(account: &String, conn: &mut SqliteConnection) -> DealsByAccount {
    let deals = deal::table
        .filter(deal::account.eq(account))
        .inner_join(inventory::table.on(inventory::vin.eq(deal::inventory)))
        .select((deal::id, deal::date, inventory::make))
        .order_by(deal::date.desc())
        .load(conn)
        .unwrap()
        ;

    deals
}
