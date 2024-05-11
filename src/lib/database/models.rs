// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::lib::database::schema::*;
use crate::lib::date::get_today::get_today;
use crate::lib::inventory::nhtsa::NHTSALookup;
use diesel::prelude::*;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Queryable, Identifiable, Associations, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = account)]
#[diesel(belongs_to(Person, foreign_key = contact))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: String,
    pub contact: String,
    pub cosigner: Option<String>,
    pub date_of_birth: Option<String>,
    pub license_number: String,
    pub license_expiration: Option<String>,
    pub date_added: Option<String>,
    pub date_modified: Option<String>,
    pub current_standing: Option<String>,
    pub notes: Option<String>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            id: String::new(),
            contact: String::new(),
            cosigner: None,
            date_of_birth: None,
            license_number: String::new(),
            license_expiration: None,
            date_added: None,
            date_modified: None,
            current_standing: None,
            notes: None,
        }
    }
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = charge)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Charge {
    pub id: String,
    pub name: String,
    pub amount: String,
    pub date_effective: String,
}

#[derive(Queryable, Debug, Selectable, PartialEq, Identifiable, Associations)]
#[diesel(table_name = creditor)]
#[diesel(belongs_to(Person, foreign_key = contact))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Creditor {
    pub id: String,
    pub business_name: String,
    pub contact: String,
    pub filing_fees: String,
    pub date_added: Option<String>,
    pub date_modified: Option<String>,
    pub apr: String,
}

#[derive(Queryable, Identifiable, Associations, Selectable, PartialEq, Debug)]
#[diesel(table_name = deal)]
#[diesel(belongs_to(Inventory, foreign_key = inventory))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Deal {
    pub id: String,
    pub state: i32,
    pub date: String,
    pub account: String,
    pub inventory: String,
    pub creditor: Option<String>,
    pub cash: String,
    pub down: Option<String>,
    pub apr: String,
    pub finance: Option<String>,
    pub lien: Option<String>,
    pub pmt: Option<String>,
    pub term: String,
    pub tax_city: Option<String>,
    pub tax_state: Option<String>,
    pub tax_county: Option<String>,
    pub tax_rtd: Option<String>,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = deal_charge)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DealCharge {
    pub deal: Option<String>,
    pub charge: Option<String>,
    pub date: Option<String>,
    pub note: Option<String>,
    pub id: String,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = deal_salesman)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DealSalesman {
    pub id: String,
    pub deal: String,
    pub salesman: String,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = deal_trade)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DealTrade {
    pub id: String,
    pub deal: String,
    pub vin: String,
    pub value: String,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = default_charge)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DefaultCharge {
    pub id: String,
    pub creditor: String,
    pub charge: String,
}

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Clone, Default)]
#[diesel(table_name = inventory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Inventory {
    pub id: String,
    pub vin: String,
    pub year: String,
    pub make: String,
    pub model: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub fuel: Option<String>,
    pub cwt: Option<String>,
    pub mileage: Option<String>,
    pub date_added: Option<String>,
    pub date_modified: Option<String>,
    pub picture: Option<String>,
    pub cash: Option<String>,
    pub credit: Option<String>,
    pub down: Option<String>,
    pub state: i32,
}

#[derive(Deserialize, Insertable, Debug, PartialEq, Clone, AsChangeset)]
#[diesel(table_name = inventory)]
pub(crate) struct SanitizedInventory {
    pub id: String,
    pub vin: String,
    pub year: String,
    pub make: String,
    pub model: String,
    pub body: String,
    pub color: String,
    pub fuel: String,
    pub cwt: String,
    pub mileage: String,
    pub cash: String,
    pub credit: String,
    pub down: String,
    pub state: i32,
    pub date_modified: Option<String>,
}

impl SanitizedInventory {
    pub(crate) fn default() -> SanitizedInventory {
        SanitizedInventory {
            id: String::new(),
            vin: String::new(),
            year: String::new(),
            make: String::new(),
            model: String::new(),
            body: String::new(),
            color: String::new(),
            fuel: String::from("GAS"),
            cwt: String::new(),
            mileage: String::new(),
            cash: "0.0".to_string(),
            credit: "0.0".to_string(),
            down: "0.0".to_string(),
            state: 1,
            date_modified: Option::from(get_today().to_string()),
        }
    }

    pub(crate) fn to_inventory(&self) -> Inventory {
        Inventory {
            id: self.id.clone(),
            vin: self.vin.clone(),
            year: self.year.clone(),
            make: self.make.clone(),
            model: Some(self.model.clone()),
            body: Some(self.body.clone()),
            color: Some(self.color.clone()),
            fuel: Some(self.fuel.clone()),
            cwt: Some(self.cwt.clone()),
            mileage: Some(self.mileage.clone()),
            date_added: None,
            date_modified: None,
            picture: None,
            cash: Some(self.cash.clone()),
            credit: Some(self.credit.clone()),
            down: Some(self.down.clone()),
            state: self.state,
        }
    }

    pub(crate) fn with_lookup(self, vehicle: NHTSALookup) -> SanitizedInventory {
        let mut current = self.clone();
        current.make = vehicle.make;
        current.model = vehicle.model;
        current.year = vehicle.year;
        current
    }

    pub(crate) fn format(&self) -> String {
        Inventory::format(&self.to_inventory())
    }
}

impl Inventory {
    pub(crate) fn format(&self) -> String {
        let make = match &self.make == "none" {
            true => "",
            false => &self.make,
        };

        let model = match &self.model {
            Some(x) if x != "none" => x.to_owned(),
            _ => String::new(),
        };

        let year = match &self.year == "none" {
            true => String::new(),
            false => {
                let year = self.year.to_string();
                let year_split = year.split(".");
                year_split.clone().nth(0).unwrap().to_string()
            }
        };

        let color = match &self.color {
            Some(x) if x != "none" => x.to_owned(),
            _ => String::new(),
        };

        let vin_last_four = match &self.vin.len() > &4 {
            true => self
                .vin
                .chars()
                .skip(self.vin.len() - 4)
                .collect::<String>(),
            false => self.vin.to_owned(),
        };

        let formatted = format!("{} {} {} {} {}", make, model, year, color, vin_last_four);
        formatted.trim().to_string().to_uppercase()
    }

    pub(crate) fn sanitize(&self) -> SanitizedInventory {
        let model = match self.model.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => String::new(),
        };

        let color = match self.color.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => String::new(),
        };

        let mileage = match self.mileage.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => String::new(),
        };

        let fuel = match self.fuel.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => String::from("GAS"),
        };

        let cwt = match self.cwt.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => String::new(),
        };

        let cash = match self.cash.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => "0.0".to_string(),
        };

        let credit = match self.credit.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => "0.0".to_string(),
        };

        let down = match self.down.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => "0.0".to_string(),
        };

        let body = match self.body.to_owned() {
            Some(x) if x != "none" && x != "null" => x,
            _ => String::new(),
        };

        // If the year includes a decimal, strip.
        let year = match self.year.find("0") {
            None => self.year.to_owned(),
            Some(_) => {
                let year = self.year.to_string();
                let year_split = year.split(".");
                year_split.clone().nth(0).unwrap().to_string()
            }
        };

        SanitizedInventory {
            id: self.id.clone().to_uppercase(),
            vin: self.vin.clone().to_uppercase(),
            year,
            make: self.make.clone().to_uppercase(),
            model: model.to_uppercase(),
            body: body.to_uppercase(),
            color: color.to_uppercase(),
            fuel: fuel.to_uppercase(),
            cwt,
            mileage,
            cash,
            credit,
            down,
            state: self.state.clone(),
            date_modified: Option::from(get_today().to_string()),
        }
    }
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = key)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Key {
    pub id: String,
    pub hashed_password: Option<String>,
    pub user_id: String,
}

#[derive(Queryable, Debug, Insertable, Selectable, Associations, PartialEq, Identifiable)]
#[diesel(table_name = payment)]
#[diesel(belongs_to(Deal, foreign_key = deal))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Payment {
    pub id: String,
    pub deal: String,
    pub date: String,
    pub amount: String,
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = payment)]
pub struct PaymentForm {
    pub deal: String,
    pub date: String,
    pub amount: String,
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Selectable, Identifiable, PartialEq, Clone)]
#[diesel(table_name = person)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: String,
    pub name_prefix: Option<String>,
    pub first_name: String,
    pub middle_initial: Option<String>,
    pub last_name: String,
    pub name_suffix: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub address_3: Option<String>,
    pub city: String,
    pub state_province: String,
    pub zip_postal: String,
    pub zip_4: Option<String>,
    pub country: String,
    pub phone_primary: String,
    pub phone_secondary: Option<String>,
    pub phone_tertiary: Option<String>,
    pub email_primary: Option<String>,
    pub email_secondary: Option<String>,
}

#[derive(
    Deserialize, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Selectable, Clone,
)]
#[diesel(table_name = person)]
pub struct PersonForm {
    pub id: String,
    pub name_prefix: String,
    pub first_name: String,
    pub middle_initial: String,
    pub last_name: String,
    pub name_suffix: String,
    pub address_1: String,
    pub address_2: String,
    pub address_3: String,
    pub city: String,
    pub state_province: String,
    pub zip_postal: String,
    pub zip_4: String,
    pub country: String,
    pub phone_primary: String,
    pub phone_secondary: String,
    pub phone_tertiary: String,
    pub email_primary: String,
    pub email_secondary: String,
}

impl PersonForm {
    pub fn to_person(self) -> Person {
        Person {
            id: self.id,
            name_prefix: Some(self.name_prefix),
            first_name: self.first_name,
            middle_initial: Some(self.middle_initial),
            last_name: self.last_name,
            name_suffix: Some(self.name_suffix),
            address_1: self.address_1,
            address_2: Some(self.address_2),
            address_3: Some(self.address_3),
            city: self.city,
            state_province: self.state_province,
            zip_postal: self.zip_postal,
            zip_4: Some(self.zip_4),
            country: self.country,
            phone_primary: self.phone_primary,
            phone_secondary: Some(self.phone_secondary),
            phone_tertiary: Some(self.phone_tertiary),
            email_primary: Some(self.email_primary),
            email_secondary: Some(self.email_secondary),
        }
    }
}

impl Person {
    pub fn address_pretty(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}, {} {}",
            self.address_1,
            self.address_2.clone().unwrap_or_default(),
            self.address_3.clone().unwrap_or_default(),
            self.city,
            self.state_province,
            self.zip_postal
        )
    }

    pub fn address(&self) -> String {
        let address_line_1 = vec![
            self.address_1.clone(),
            self.address_2.clone().unwrap_or_default(),
            self.address_3.clone().unwrap_or_default(),
        ]
        .join(" ")
        .trim()
        .to_string();

        // street [PO Box, etc], city, state zip

        let address_parts = vec![
            address_line_1,
            self.city.clone(),
            format!(
                "{} {}",
                self.state_province.clone(),
                self.zip_postal.clone(),
            ),
        ];

        address_parts.join(", ").trim().to_string()
    }
}

impl Default for Person {
    fn default() -> Self {
        Person {
            id: String::new(),
            name_prefix: None,
            first_name: String::new(),
            middle_initial: None,
            last_name: String::new(),
            name_suffix: None,
            address_1: String::new(),
            address_2: None,
            address_3: None,
            city: String::from("Fort Morgan"),
            state_province: String::from("CO"),
            zip_postal: String::from("80701"),
            zip_4: None,
            country: String::new(),
            phone_primary: String::new(),
            phone_secondary: None,
            phone_tertiary: None,
            email_primary: None,
            email_secondary: None,
        }
    }
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = person)]
pub struct PersonName {
    pub name_prefix: Option<String>,
    pub first_name: String,
    pub middle_initial: Option<String>,
    pub last_name: String,
    pub name_suffix: Option<String>,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = salesman)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Salesman {
    pub id: String,
    pub person: String,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = session)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub active_expires: i64,
    pub idle_expires: i64,
    pub iv: Option<String>,
}

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}
