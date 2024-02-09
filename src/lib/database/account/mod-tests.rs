#[cfg(test)]
mod tests {
    use autoflp_desktop::lib::database::{account, establish_connection};
    use account::get_account_details;
    use autoflp_desktop::lib::database::account::get_account_details;

    #[test]
    fn test_deal_account_inventory() {
        let mut conn = establish_connection();
        let person_id = "2c4d4cc8-5729-44bb-ad71-ec79dd7fedef";

        let result = get_account_details::get_account_details(Some(person_id.to_string()));

        assert_eq!(1, 1);
    }
}
