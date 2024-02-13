use crate::lib::database::models::{Creditor, Deal, Inventory, Payment};

pub mod get_deal;
pub mod get_deal_payments;
pub mod get_details;
pub mod get_deals_by_account;
pub type AccountDeal = (Deal, Inventory, Option<Creditor>);
pub type AccountDeals = Vec<AccountDeal>;
pub type DealDetails = (Deal, Inventory, Option<Creditor>);
pub type PaymentsByDeal = (Deal, Inventory, Option<Creditor>, Vec<Payment>);
pub type DealByAccount = (String, String, String, i32);
pub type DealsByAccount = Vec<DealByAccount>;
