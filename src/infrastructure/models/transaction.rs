use crate::{
	domain::models::transaction::{status::TransactionStatus, txn_type::TransactionType, CreateTransaction, Transaction, UpdateTransaction},
	infrastructure::schema::transactions,
};
use diesel::{AsChangeset, Insertable, Queryable};
use serde_json::Value;

#[derive(Queryable, Debug)]
pub struct TransactionDiesel {
	pub transaction_id: i32,
	pub user_id: Option<i32>,
	pub transaction_type_id: Option<i32>,
	pub transaction_data: Value,
	pub transaction_hash: String,
	pub transaction_date: chrono::NaiveDateTime,
	pub block_number: Option<i32>,
	pub status: String,
}

impl From<Transaction> for TransactionDiesel {
	fn from(transaction: Transaction) -> Self {
		TransactionDiesel {
			transaction_id: transaction.transaction_id,
			user_id: transaction.user_id,
			transaction_type_id: Some(transaction.transaction_type as i32),
			transaction_data: transaction.transaction_data,
			transaction_hash: transaction.transaction_hash,
			transaction_date: transaction.transaction_date,
			block_number: transaction.block_number,
			status: transaction.status.to_string(),
		}
	}
}

impl Into<Transaction> for TransactionDiesel {
	fn into(self) -> Transaction {
		Transaction {
			transaction_id: self.transaction_id,
			user_id: self.user_id,
			transaction_type: TransactionType::from(self.transaction_type_id.unwrap_or(0)),
			transaction_data: self.transaction_data,
			transaction_hash: self.transaction_hash,
			transaction_date: self.transaction_date,
			block_number: self.block_number,
			status: self.status.parse::<TransactionStatus>().unwrap_or(TransactionStatus::PENDING),
		}
	}
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct CreateTransactionDiesel {
	pub user_id: i32,
	pub transaction_type_id: i32,
	pub transaction_data: Value,
	pub transaction_hash: String,
}

impl From<CreateTransaction> for CreateTransactionDiesel {
	fn from(transaction: CreateTransaction) -> Self {
		CreateTransactionDiesel {
			user_id: transaction.user_id,
			transaction_type_id: transaction.transaction_type as i32,
			transaction_data: transaction.transaction_data,
			transaction_hash: transaction.transaction_hash,
		}
	}
}

impl Into<CreateTransaction> for CreateTransactionDiesel {
	fn into(self) -> CreateTransaction {
		CreateTransaction {
			user_id: self.user_id,
			transaction_type: TransactionType::from(self.transaction_type_id),
			transaction_data: self.transaction_data,
			transaction_hash: self.transaction_hash,
		}
	}
}

#[derive(AsChangeset)]
#[diesel(table_name = transactions)]
pub struct UpdateTransactionDiesel {
	pub block_number: i32,
	pub status: String,
}

impl From<UpdateTransaction> for UpdateTransactionDiesel {
	fn from(transaction: UpdateTransaction) -> Self {
		UpdateTransactionDiesel { block_number: transaction.block_number, status: transaction.status.to_string() }
	}
}
