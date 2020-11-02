//! Release model module

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    #[serde(rename(serialize = "url"))]
    pub html_url: String,
    pub body: String,
    pub created_at: String,
    pub published_at: String,
}
