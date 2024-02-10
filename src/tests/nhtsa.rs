#[cfg(test)]
mod tests {
    use tokio_test::block_on;

    use crate::lib::inventory::nhtsa::{fetch_nhtsa, format_nhtsa, get_vehicle_info};

    macro_rules! aw {
    ($e:expr) => {
        block_on($e)
    };
  }
    #[test]
    fn test_fetch_nhtsa_vin() {
        let vin = "1B7GG23Y1NS526835".to_string();

        let response = fetch_nhtsa(vin);

        assert!(aw!(response).is_ok());
    }

    #[test]
    fn test_format_nhtsa_response() {
        let vin = "1B7GG23Y1NS526835".to_string();

        let response = fetch_nhtsa(vin);
        let json = aw!(response).unwrap();
        let vehicle = format_nhtsa(json);

        assert!(vehicle.is_ok());

        assert_eq!(vehicle.unwrap().make, "DODGE");
    }

    #[test]
    fn test_get_vehicle_info() {
        let vin = "1B7GG23Y1NS526835".to_string();

        let vehicle = aw!(get_vehicle_info(vin));

        assert_eq!(vehicle.unwrap().make, "DODGE");
    }

    #[test]
    fn test_bad_vehicle_lookup() {
        let vin = "000".to_string();

        let vehicle = aw!(get_vehicle_info(vin));
        assert_eq!(vehicle.is_ok(), false);
    }
}