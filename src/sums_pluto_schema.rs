
use serde::{Deserialize, Serialize};

pub type Pages = Vec<Page>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub(crate) current_page: i64,
    pub(crate) data: Vec<Event>,
    #[serde(default)]
    pub(crate) first_page_url: String,
    pub(crate) from: i64,
    pub(crate) to: i64,
    #[serde(default)]
    pub(crate) path: String,
    #[serde(default)]
    pub(crate) per_page: String,
    pub(crate) next_page_url: Option<String>,
    pub(crate) prev_page_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub(crate) id: i64,
    pub(crate) event_id: i64,
    #[serde(default)]
    pub(crate) title: String,
    pub(crate) event_date_title: Option<String>,
    pub(crate) url_name: Option<String>,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
    pub(crate) doors_open_at: Option<serde_json::Value>,
    pub(crate) external_tickets: Option<String>,
    pub(crate) thumbnail_url: Option<String>,
    pub(crate) app_thumbnail_url: Option<String>,
    pub(crate) image_url: Option<String>,
    pub(crate) short_description: Option<String>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub(crate) description: String,
    pub(crate) group: Option<Box<Group>>,
    pub(crate) venue: Option<Venue>,
    pub(crate) age: Option<Type>,
    #[serde(rename = "type")]
    pub(crate) datum_type: Type,
    pub(crate) has_products: i64,
    pub(crate) categories: Vec<Category>,
    pub(crate) accessibilities: Vec<Option<serde_json::Value>>,
    pub(crate) premium: Option<serde_json::Value>,
    pub(crate) unlisted: i64,
    pub(crate) product_count: i64,
    pub(crate) hidden_product_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    pub(crate) id: i64,
    #[serde(default)]
    pub(crate) name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub(crate) id: i64,
    #[serde(default)]
    pub(crate) name: String,
    pub(crate) order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub(crate) id: i64,
    pub(crate) name: Option<String>,
    pub(crate) thumbnail_url: Option<String>,
    pub(crate) app_thumbnail_url: Option<serde_json::Value>,
    pub(crate) category: Option<Box<Group>>,
    pub(crate) parent: Option<Box<Group>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub(crate) id: i64,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) address1: String,
    #[serde(default)]
    pub(crate) address2: String,
    #[serde(default)]
    pub(crate) address3: String,
    #[serde(default)]
    pub(crate) address4: String,
    #[serde(default)]
    pub(crate) postcode: String,
    #[serde(default)]
    pub(crate) country: String,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
