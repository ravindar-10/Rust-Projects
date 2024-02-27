use chrono;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User
{
	pub user_id: i32,
	pub email: String,
	pub first_name: String,
	pub last_name: String,
	pub nonce: i32,
	pub registration_date: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateUser
{
	pub email: String,
	pub first_name: String,
	pub last_name: String,
	pub uuid: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateUser
{
	pub email: String,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
}
