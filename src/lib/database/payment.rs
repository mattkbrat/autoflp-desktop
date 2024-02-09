use diesel::{QueryDsl, RunQueryDsl};
use crate::lib::database;
use crate::lib::database::models::{Payment, PaymentForm};
use crate::lib::database::schema::{payment};
use uuid::Uuid;

pub fn add_payment(new_payment_data: &PaymentForm) -> Result<i32, String> {

    let mut conn = database::establish_connection();

    let pmt = Payment {
        amount: new_payment_data.amount.to_string(),
        deal: new_payment_data.deal.to_string(),
        date: new_payment_data.date.to_string(),
        id: Uuid::new_v4().to_string(),
    };


    let new_payment = diesel::insert_into(payment::table)
        .values(
            pmt
        )
        .execute(&mut conn);


    if new_payment.is_ok() {
        println!("{} new payment inserted", new_payment.unwrap());
        return Ok(200);
    }

    let error_message = "Could not insert: ".to_owned() + &new_payment.unwrap_err().to_string();

    return Err(error_message);
}

pub fn get_payments() -> Vec<Payment> {
    let mut conn = database::establish_connection();

    database::schema::payment::table
        .load::<Payment>(&mut conn)
        .expect("Error loading payments")
}

pub fn get_payment(payment_id: &str) -> Option<Payment> {
    let mut conn = database::establish_connection();

    let payment = database::schema::payment::table
        .find(payment_id)
        .first(&mut conn);

    match payment {
        Ok(p) => Some(p),
        Err(_) => None
    }
}

pub fn delete_payment(payment_id: &str) -> Result<i32, String> {
    let mut conn = database::establish_connection();

    let deleted_payment = diesel::delete(database::schema::payment::table.find(payment_id))
        .execute(&mut conn);

    if deleted_payment.is_ok() {
        println!("{} payment deleted", deleted_payment.unwrap());
        return Ok(200);
    }

    let error_message = "Could not insert: ".to_owned() + &deleted_payment.unwrap_err().to_string();

    return Err(error_message);
}