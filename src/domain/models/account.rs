use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Account {
	pub account_id: i32,
	pub account_number: String,
	pub user_id: Option<i32>,
	pub balance: f64,
	pub account_type_id: Option<i32>,
	pub latest_transaction_hash: Option<String>,
	pub created_date: chrono::NaiveDateTime,
	pub updated_date: chrono::NaiveDateTime,
	pub is_deleted: bool,
}

#[derive(Clone)]
pub struct CreateAccount {
	pub account_number: String,
	pub user_id: i32,
	pub account_type_id: i32,
	pub latest_transaction_hash: String,
}

#[derive(Clone)]
pub struct UpdateAccount {
	pub balance: Option<f64>,
	pub latest_transaction_hash: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DeleteAccount {
	pub account_num: String,
	pub is_deleted: bool,
}
