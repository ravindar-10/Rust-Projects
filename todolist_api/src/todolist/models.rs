use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}
#[derive(Deserialize, Serialize)]
pub struct UpdateEntryData {
    pub title: String,
}
