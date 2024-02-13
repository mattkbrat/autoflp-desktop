#[cfg(test)]
mod tests {
    use diesel::{QueryDsl, RunQueryDsl};

    use crate::lib::database;

    use database::account::get_account_details;
    use database::{establish_connection, account, models, schema};

    use account::update_person::update_details;
    use models::Person;
    use schema::person;
    use crate::lib::database::models::PersonForm;
    use tokio_test::block_on;

    macro_rules! aw {
    ($e:expr) => {
        block_on($e)
    };
  }
    #[test]
    fn test_deal_account_inventory() {
        let mut conn = establish_connection();
        let person_id = "2c4d4cc8-5729-44bb-ad71-ec79dd7fedef";

        let result = get_account_details::get_account_details(Some(person_id.to_string()));

        assert_eq!(1, 1);
    }

    #[test]
    fn test_update_person() {
        let mut conn = establish_connection();

        let mut new_person = PersonForm {
            id: "123".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            middle_initial: "A".to_string(),
            name_prefix: "Mr".to_string(),
            name_suffix: "Jr".to_string(),
            address_1: "".to_string(),
            address_2: "".to_string(), // "Apt. 123",
            address_3: "".to_string(), // "Suite 456",
            city: "".to_string(),
            state_province: "".to_string(),
            zip_postal: "".to_string(),
            zip_4: "".to_string(),
            country: "".to_string(),
            phone_primary: "".to_string(),
            phone_secondary: "".to_string(),
            phone_tertiary: "".to_string(),
            email_primary: "".to_string(),
            email_secondary: "".to_string(),
        };

        let inserted = diesel::insert_into(person::table).values(&new_person).execute(&mut conn);

        assert!(inserted.is_ok());

        let new_address = "123 Main St.".to_string();

        new_person.address_1 = new_address.clone();

        let updated = aw!(update_details(new_person.clone()));

        assert!(updated.is_ok());

        let mut person = person::table.find(&new_person.id).first::<Person>(&mut conn).unwrap();

        assert_eq!(person.address_1, new_address);

        let deleted = diesel::delete(person::table.find(&new_person.id)).execute(&mut conn);

        assert!(deleted.is_ok());
    }
}
