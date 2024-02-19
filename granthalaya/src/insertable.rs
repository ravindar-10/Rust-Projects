use crate::schema::books;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Serialize, Clone, Deserialize)]
#[diesel(table_name=books)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub publication: Option<String>,
    pub year: Option<i32>,
}
