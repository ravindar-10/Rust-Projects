use chrono;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::domain::models::account::{Account, DeleteAccount};

#[derive(Debug, Clone, Validate, Serialize, Deserialize, ToSchema)]
pub struct AccountDTO {
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

impl From<Account> for AccountDTO {
	fn from(account: Account) -> AccountDTO {
		AccountDTO {
			account_id: account.account_id,
			account_number: account.account_number,
			user_id: account.user_id,
			balance: account.balance,
			account_type_id: account.account_type_id,
			latest_transaction_hash: account.latest_transaction_hash,
			created_date: account.created_date,
			updated_date: account.updated_date,
			is_deleted: account.is_deleted,
		}
	}
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, ToSchema)]
pub struct DeleteAccountDTO {
	pub account_num: String,
	pub is_deleted: bool,
}

impl Into<DeleteAccount> for DeleteAccountDTO {
	fn into(self) -> DeleteAccount {
		DeleteAccount { account_num: self.account_num, is_deleted: self.is_deleted }
	}
}
