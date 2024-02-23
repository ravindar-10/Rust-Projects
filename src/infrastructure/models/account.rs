use crate::{
	domain::models::account::{Account, CreateAccount, DeleteAccount, UpdateAccount},
	infrastructure::schema::accounts,
};
use diesel::{AsChangeset, Insertable, Queryable};

#[derive(Queryable)]
pub struct AccountDiesel {
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

impl Into<Account> for AccountDiesel {
	fn into(self) -> Account {
		Account {
			account_id: self.account_id,
			account_number: self.account_number,
			user_id: self.user_id,
			balance: self.balance,
			account_type_id: self.account_type_id,
			latest_transaction_hash: self.latest_transaction_hash,
			created_date: self.created_date,
			updated_date: self.updated_date,
			is_deleted: self.is_deleted,
		}
	}
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct CreateAccountDiesel {
	pub account_number: String,
	pub user_id: i32,
	pub account_type_id: i32,
	pub latest_transaction_hash: String,
}

impl From<CreateAccount> for CreateAccountDiesel {
	fn from(t: CreateAccount) -> Self {
		CreateAccountDiesel {
			account_number: t.account_number,
			user_id: t.user_id,
			account_type_id: t.account_type_id,
			latest_transaction_hash: t.latest_transaction_hash,
		}
	}
}

#[derive(AsChangeset)]
#[diesel(table_name = accounts)]
pub struct UpdateAccountDiesel {
	pub balance: Option<f64>,
	pub latest_transaction_hash: String,
}

impl From<UpdateAccount> for UpdateAccountDiesel {
	fn from(t: UpdateAccount) -> Self {
		UpdateAccountDiesel { balance: t.balance, latest_transaction_hash: t.latest_transaction_hash }
	}
}

#[derive(AsChangeset)]
#[diesel(table_name = accounts)]
pub struct DeleteAccountDiesel {
	pub is_deleted: bool,
}

impl From<DeleteAccount> for DeleteAccountDiesel {
	fn from(t: DeleteAccount) -> Self {
		DeleteAccountDiesel { is_deleted: t.is_deleted }
	}
}
