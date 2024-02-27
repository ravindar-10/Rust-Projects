use serde::Deserialize;
use serde_json::Value;

use crate::domain::models::transaction::{status::TransactionStatus, txn_type::TransactionType};

#[derive(Clone, Deserialize, Debug)]
pub struct Transaction
{
	pub transaction_id: i32,
	pub user_id: Option<i32>,
	pub transaction_type: TransactionType,
	pub transaction_data: Value,
	pub transaction_hash: String,
	pub transaction_date: chrono::NaiveDateTime,
	pub block_number: Option<i32>,
	pub status: TransactionStatus,
	pub uuid: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateTransaction
{
	pub user_id: i32,
	pub transaction_type: TransactionType,
	pub transaction_data: Value,
	pub transaction_hash: String,
	pub uuid: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateTransaction
{
	pub block_number: i32,
	pub status: TransactionStatus,
}

pub mod deposit;
pub mod status;
pub mod transfer;
pub mod txn_type;
pub mod withdrawal;
