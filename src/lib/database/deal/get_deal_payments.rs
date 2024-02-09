use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use crate::lib::database::models::{Deal, Payment};
use crate::lib::database::schema::payment;

pub fn get_deal_payments(deal: &Deal, conn: &mut SqliteConnection) -> Vec<Payment> {

    let payments = Payment::belonging_to(&deal)
        .select(Payment::as_select())
        .order_by(payment::date.desc())
        .load(conn)
        .unwrap()
        ;

    payments
}
