use crate::lib;
use lib::pushover::{Priority, Sound, message};
use message::NewMessage;
use std::env;
use crate::lib::pushover::message::TransactionType;


pub struct PaymentMessage {
    pub amount: f64,
    pub currency: String,
    pub account: String,
}

pub async fn notify_payment(p: PaymentMessage, t: TransactionType) -> Result<i32, String> {
    let message_body = match t {
        TransactionType::New => format!("{}{} credited towards {}", p.currency, p.amount, p.account),
        TransactionType::Delete => format!("{}{} debited against {}", p.currency, p.amount, p.account),
    };

    let message_title = match t {
        TransactionType::New => "Payment received",
        TransactionType::Delete => "Payment deleted",
    };

    let message = NewMessage::new(
        env::var("PUSHOVER_TOKEN").expect("PUSHOVER_TOKEN must be set"),
        env::var("PUSHOVER_USER").expect("PUSHOVER_USER must be set"),
        message_body
    ).title(String::from(message_title)).priority(Priority::Normal).sound(Sound::Pushover);

    lib::pushover::send::send_message(message).await
}
