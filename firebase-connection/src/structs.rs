use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub name: String,
}