use std::any::type_name;
use std::env;

use dotenvy::dotenv;
use url::Url;

use crate::lib::unsplash::structs::UnsplashResults;

fn type_of<T>(_: T) -> &'static str {
    // (DevinR528, https://users.rust-lang.org/t/how-check-type-of-variable/33845/2)
    type_name::<T>()
}

pub(crate) async fn fetch_unsplash(query: &str, count: u8) -> Result<UnsplashResults, String> {

    dotenv().ok();

    let content_filter = "high".to_string();
    let client_id = env::var("UNSPLASH_ACCESS_KEY").expect("UNSPLASH_ACCESS_KEY must be set");
    let orientation = "landscape".to_string();
    let base_url = "https://api.unsplash.com/photos/random?";


    let url = Url::parse_with_params(&base_url,
                                     &[("content_filter", &content_filter), ("client_id", &client_id,), ("orientation", &orientation),
                                         ("topics", &query.to_string()), ("count", &count.to_string())
                                     ]).expect("Failed to parse URL");

    println!("url: {}", url.as_str());
    let response = reqwest::get(url).await.expect("Failed to fetch URL");

    if !response.status().is_success() {
        // panic!("Failed to fetch URL: {}", response.status());
        return Err(format!("Failed to fetch URL: {}", response.status()));
    }

    let root = response.json::<UnsplashResults>().await;

    if !root.is_ok() {
        println!("Failed to parse JSON: {:?}", root);
        return Err(format!("Failed to parse JSON: {:?}", root));
    }

    Ok(root.unwrap())
}

#[cfg(test)]
mod tests {
    use tokio_test::block_on;

    use super::*;

    macro_rules! aw {
    ($e:expr) => {
        block_on($e)
    };
        }

    #[test]
    fn test_fetch() {
        let result = aw!(fetch_unsplash(&"automobile", 1));
        if !&result.is_ok() {
            println!("Test failed, {:?}", result.clone().unwrap_err())
        }
        assert_eq!(&result.is_ok(), &true);
        let unwrapped = result.unwrap();
        assert_eq!(type_of(&unwrapped[0].alt_description), "&core::option::Option<alloc::string::String>");
    }
}