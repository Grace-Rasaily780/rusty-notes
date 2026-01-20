use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
    pub body: String,
}
