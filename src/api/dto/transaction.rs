use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::domain::{
	models::transaction::{status::TransactionStatus, txn_type::TransactionType, CreateTransaction, Transaction},
	repositories::repository::ResultPaging,
};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTransactionDTO
{
	pub user_id: i32,
	pub transaction_type_str: String,
	pub transaction_data: Value,
	pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionDTO
{
	pub transaction_id: i32,
	pub user_id: i32,
	pub transaction_type_str: String,
	pub transaction_data: Value,
	pub transaction_hash: String,
	pub transaction_date: NaiveDateTime,
	pub block_number: Option<i32>,
	pub status: TransactionStatus,
}

impl Into<TransactionDTO> for Transaction
{
	fn into(self) -> TransactionDTO
	{
		TransactionDTO {
			transaction_id: self.transaction_id,
			user_id: self.user_id.unwrap(),
			transaction_type_str: self.transaction_type.to_string(),
			transaction_data: self.transaction_data,
			transaction_hash: self.transaction_hash,
			transaction_date: self.transaction_date,
			block_number: self.block_number,
			status: self.status,
		}
	}
}

impl Into<CreateTransaction> for CreateTransactionDTO
{
	fn into(self) -> CreateTransaction
	{
		CreateTransaction {
			user_id: self.user_id,
			transaction_type: TransactionType::from_str(&self.transaction_type_str).unwrap(),
			transaction_data: self.transaction_data,
			transaction_hash: "".to_string(), // Hash to be computed at service layer
			uuid: self.uuid,
		}
	}
}

impl Into<ResultPaging<TransactionDTO>> for ResultPaging<Transaction>
{
	fn into(self) -> ResultPaging<TransactionDTO>
	{
		ResultPaging { total: self.total, items: self.items.into_iter().map(|transaction| transaction.into()).collect() }
	}
}
