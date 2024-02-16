use serde_derive::Deserialize;
use serde_derive::Serialize;

const QUOTABLE_URL: &str = "https://api.quotable.io/quotes/random?limit=1";


pub type Root = Vec<Quotable>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quotable {
    #[serde(rename = "_id")]
    pub id: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<String>,
    pub author_slug: String,
}

impl Default for Quotable {
    fn default() -> Self {
        Quotable {
            content: "The only thing we have to fear is fear itself.".to_string(),
            author: "Franklin D. Roosevelt".to_string(),
            tags: vec!["fear".to_string(), "inspirational".to_string()],
            author_slug: "franklin-d-roosevelt".to_string(),
            id: String::new(),
        }
    }
}

pub(crate) async fn fetch_quotable() -> Result<Quotable, String> {
    let response = reqwest::get(QUOTABLE_URL).await;

    if !response.is_ok() {
        let error = response.unwrap_err();
        return Err(error.to_string());
    }

    let response = response.unwrap();

    let quote: Result<Root, _> = response.json().await;
    if (quote.is_ok()) {
        return Ok(quote.unwrap()[0].clone());
    } else {
        let error_message = quote.unwrap_err();
        return Err(error_message.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    macro_rules! aw {
        ($e:expr) => {
            block_on($e)
        };
    }

    #[test]
    fn test_fetch_quotable() {
        let quote = aw!(fetch_quotable()).unwrap();
        assert_eq!(quote.content.is_empty(), false);
        assert_eq!(quote.author.is_empty(), false);
    }
}
