// generated with https://transform.tools/json-to-rust-serde

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    // pub id: String,
    // pub slug: String,
    // #[serde(rename = "created_at")]
    // pub created_at: String,
    // #[serde(rename = "updated_at")]
    // pub updated_at: String,
    // #[serde(rename = "promoted_at")]
    // pub promoted_at: Option<String>,
    pub width: i32,
    pub height: i32,
    pub color: String,
    #[serde(rename = "blur_hash")]
    // pub blur_hash: String,
    pub description: Option<String>,
    #[serde(rename = "alt_description")]
    pub alt_description: Option<String>,
    // pub breadcrumbs: Vec<Value>,
    pub urls: Urls,
    pub links: Links,
    // pub likes: i64,
    // #[serde(rename = "liked_by_user")]
    // pub liked_by_user: bool,
    // #[serde(rename = "current_user_collections")]
    // pub current_user_collections: Vec<Value>,
    // pub sponsorship: Value,
    // #[serde(rename = "topic_submissions")]
    // pub topic_submissions: TopicSubmissions,
    pub user: User,
    // pub exif: Exif,
    // pub location: Location,
    // pub views: i64,
    // pub downloads: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
    #[serde(rename = "small_s3")]
    pub small_s3: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub html: String,
    pub download: String,
    #[serde(rename = "download_location")]
    pub download_location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicSubmissions {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    // pub id: String,
    // #[serde(rename = "updated_at")]
    // pub updated_at: String,
    // pub username: String,
    pub name: String,
    // #[serde(rename = "first_name")]
    // pub first_name: String,
    // #[serde(rename = "last_name")]
    // pub last_name: String,
    // #[serde(rename = "twitter_username")]
    // pub twitter_username: Option<String>,
    // #[serde(rename = "portfolio_url")]
    // pub portfolio_url: Option<String>,
    // pub bio: Option<String>,
    // pub location: Option<String>,
    pub links: Links2,
    // #[serde(rename = "profile_image")]
    // pub profile_image: ProfileImage,
    // #[serde(rename = "instagram_username")]
    // pub instagram_username: String,
    // #[serde(rename = "total_collections")]
    // pub total_collections: i64,
    // #[serde(rename = "total_likes")]
    // pub total_likes: i64,
    // #[serde(rename = "total_photos")]
    // pub total_photos: i64,
    // #[serde(rename = "total_promoted_photos")]
    // pub total_promoted_photos: i64,
    // #[serde(rename = "accepted_tos")]
    // pub accepted_tos: bool,
    // #[serde(rename = "for_hire")]
    // pub for_hire: bool,
    // pub social: Social,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    // #[serde(rename = "self")]
    // pub self_field: String,
    pub html: String,
    // pub photos: String,
    // pub likes: String,
    // pub portfolio: String,
    // pub following: String,
    // pub followers: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileImage {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Social {
    #[serde(rename = "instagram_username")]
    pub instagram_username: String,
    #[serde(rename = "portfolio_url")]
    pub portfolio_url: Option<String>,
    #[serde(rename = "twitter_username")]
    pub twitter_username: Option<String>,
    #[serde(rename = "paypal_email")]
    pub paypal_email: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "exposure_time")]
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    #[serde(rename = "focal_length")]
    pub focal_length: Option<String>,
    pub iso: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub position: Position,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
