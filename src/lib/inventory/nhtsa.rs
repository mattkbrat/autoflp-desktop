use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NHTSALookup {
    pub make: String,
    pub model: String,
    pub year: String,
    pub vin: String,
}


// Return json body or error message
pub async fn fetch_nhtsa(vin: String) -> Result<Vec<Value>, String> {
    let url = format!("https://vpic.nhtsa.dot.gov/api/vehicles/decodevin/{}?format=json", vin);
    let response = reqwest::get(&url).await.unwrap();
    if !response.status().is_success() {
        let error_message = format!("Request failed with status code: {}", response.status());
        return Err(error_message.to_string());
    }

    let json = response.json::<Value>().await.unwrap();

    let results = json.get("Results").unwrap();

    if results.as_array().unwrap().len() == 0 {
        return Err("No results found".to_string());
    }

    let results = results.as_array().unwrap();

    Ok(results.to_owned())
}

pub fn format_nhtsa(value: Vec<Value>) -> Result<NHTSALookup, String> {
    let mut vehicle = HashMap::new();

    for item in value {
        let variable = item.get("Variable").unwrap().as_str();
        let value = item.get("Value").unwrap().as_str();

        if let Some(variable) = variable {
            if let Some(value) = value {
                vehicle.insert(variable.to_string(), value.to_string().to_uppercase());
            } else {
                vehicle.insert(variable.to_string(), "".to_string());
            }
        }
    }

    let unknown = "Unknown";

    let error_code = match vehicle.get("Error Code") {
        Some(error_code) => error_code.to_owned(),
        None => unknown.to_owned(),
    };

    if error_code != "0" {
        let error_message = format!("Error code {error_code} returned: {}",
                                    &*match vehicle.get("Error Text") {
                                        Some(error_message) => error_message.to_owned(),
                                        None => unknown.to_owned(),
                                    }
        ).to_owned();
        return Err(error_message.to_string());
    }

    let make = match vehicle.get("Make") {
        Some(make) => make.to_owned(),
        None => unknown.to_owned(),
    };

    let model = match vehicle.get("Model") {
        Some(model) => model.to_owned().to_uppercase(),
        None => unknown.to_owned(),
    };

    let year = match vehicle.get("Model Year") {
        Some(year) => year.to_owned(),
        None => unknown.to_owned(),
    };

    let vin = match vehicle.get("VIN") {
        Some(vin) => vin.to_owned(),
        None => unknown.to_owned(),
    };

    Ok(NHTSALookup {
        make,
        model,
        year,
        vin,
    })
}

pub async fn get_vehicle_info(vin: String) -> Result<NHTSALookup, String> {
    let response = fetch_nhtsa(vin).await;
    let json = match response {
        Ok(json) => json,
        Err(e) => return Err(e),
    };

    let vehicle = format_nhtsa(json);

    vehicle
}
