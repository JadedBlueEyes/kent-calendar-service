// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Data;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Data = serde_json::from_str(&json).unwrap();
// }

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub(crate) assets_base_url: String,
    pub(crate) event_campuses: Vec<EventCampus>,
    pub(crate) event_categories: Vec<String>,
    pub(crate) event_count: i64,
    pub(crate) event_tags: EventTags,
    pub(crate) events: Vec<Event>,
    pub(crate) events_base_url: String,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventCampus {
    Canterbury,
    #[serde(rename = "")]
    Empty,
    Medway,
}

type EventTags = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub(crate) all_day: bool,
    pub(crate) availability: String,
    pub(crate) banner_image: BannerImage,
    pub(crate) campus: i64,
    pub(crate) campus_name: EventCampus,
    pub(crate) categories: Vec<PrimaryCalendar>,
    pub(crate) categories_string: String,
    pub(crate) contact_email: String,
    pub(crate) contact_name: String,
    pub(crate) contact_phone: ContactPhone,
    pub(crate) country: Country,
    pub(crate) description: String,
    pub(crate) dynamics: String,
    pub(crate) dynamics_expiry: String,
    pub(crate) dynamics_expiry_text: String,
    pub(crate) dynamics_no_expiry: bool,
    pub(crate) end: String,
    pub(crate) end_date: String,
    pub(crate) end_day: String,
    pub(crate) end_month: Month,
    pub(crate) end_month_full: MonthFull,
    pub(crate) end_time: String,
    pub(crate) end_year: String,
    pub(crate) event_calendar: String,
    pub(crate) event_categories: Vec<String>,
    pub(crate) event_tags: String,
    pub(crate) id: i64,
    pub(crate) image: Image,
    pub(crate) index: String,
    pub(crate) intro: String,
    pub(crate) location: String,
    pub(crate) map_url: String,
    pub(crate) online_event: bool,
    pub(crate) open_to: String,
    pub(crate) pricing: String,
    pub(crate) primary_calendar: PrimaryCalendar,
    pub(crate) series_slug: SeriesSlug,
    pub(crate) series_title: SeriesTitle,
    pub(crate) slug: String,
    pub(crate) sponsor_name: String,
    pub(crate) sponsor_url: String,
    pub(crate) start: String,
    pub(crate) start_date: String,
    pub(crate) start_day: String,
    pub(crate) start_month: Month,
    pub(crate) start_month_full: MonthFull,
    pub(crate) start_time: String,
    pub(crate) start_year: String,
    pub(crate) subtitle: String,
    pub(crate) tags: Vec<PrimaryCalendar>,
    pub(crate) tentative: bool,
    pub(crate) title: String,
    pub(crate) url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannerImage {
    pub(crate) alt_text: String,
    pub(crate) attribution: Attribution,
    pub(crate) caption: String,
    pub(crate) created_at: String,
    pub(crate) deleted_at: Option<serde_json::Value>,
    pub(crate) focus: Focus,
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) primary_calendar: i64,
    pub(crate) sizes: Sizes,
    pub(crate) title: String,
    pub(crate) updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    pub(crate) author: String,
    pub(crate) license: String,
    pub(crate) link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Focus {
    Center,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sizes {
    pub(crate) full: Full,
    pub(crate) thumbnail: Thumbnail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Full {
    pub(crate) height: String,
    pub(crate) url: String,
    pub(crate) width: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub(crate) height: i64,
    pub(crate) url: String,
    pub(crate) width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryCalendar {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactPhone {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "01227764000")]
    The01227764000,
    #[serde(rename = "01227 827335")]
    The01227827335,
    #[serde(rename = "07545871087")]
    The07545871087,
    #[serde(rename = "07858872728")]
    The07858872728,
    #[serde(rename = "07921858319")]
    The07921858319,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Country {
    #[serde(rename = "United Kingdom")]
    UnitedKingdom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Month {
    Aug,
    Dec,
    Nov,
    Oct,
    Sep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonthFull {
    August,
    December,
    November,
    October,
    September,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub(crate) alt: String,
    pub(crate) height: i64,
    pub(crate) src: String,
    pub(crate) width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SeriesSlug {
    #[serde(rename = "all-levels")]
    AllLevels,
    #[serde(rename = "campus-tours")]
    CampusTours,
    #[serde(rename = "")]
    Empty,
    Icss,
    Kentcog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeriesTitle {
    #[serde(rename = "(All levels)")]
    AllLevels,
    #[serde(rename = "Campus Tours")]
    CampusTours,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "iCSS")]
    ICss,
    #[serde(rename = "KentCOG")]
    KentCog,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoints {
    pub(crate) lg: i64,
    pub(crate) md: i64,
    pub(crate) sm: i64,
    pub(crate) xl: i64,
    pub(crate) xxl: i64,
    pub(crate) xxxl: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Img {
    pub(crate) aspect_ratio: f64,
    pub(crate) created_at: String,
    pub(crate) created_by: i64,
    pub(crate) deleted_at: Option<serde_json::Value>,
    pub(crate) duration: Option<serde_json::Value>,
    pub(crate) file: Vec<Option<serde_json::Value>>,
    pub(crate) filename: String,
    pub(crate) filesize: i64,
    pub(crate) format: String,
    pub(crate) hash: String,
    pub(crate) height: i64,
    pub(crate) id: i64,
    pub(crate) mime_type: String,
    #[serde(rename = "type")]
    pub(crate) img_type: String,
    pub(crate) updated_at: String,
    pub(crate) updated_by: i64,
    pub(crate) url: String,
    pub(crate) variants: Variants,
    pub(crate) width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variants {
    #[serde(rename = "400x400")]
    pub(crate) the_400_x400: String,
    pub(crate) base64: String,
    #[serde(rename = "base64inline")]
    pub(crate) base64_inline: String,
    #[serde(rename = "base64square")]
    pub(crate) base64_square: String,
    #[serde(rename = "base64video")]
    pub(crate) base64_video: String,
}
