use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;

use crate::domain::error::RepositoryError;
use crate::domain::models::account::DeleteAccount;
use crate::infrastructure::models::account::DeleteAccountDiesel;
use crate::infrastructure::schema::accounts::is_deleted;
use crate::{
	domain::{
		models::account::{Account, CreateAccount, UpdateAccount},
		repositories::{account::AccountRepository, repository::RepositoryResult},
	},
	infrastructure::{
		databases::pgsql::DBConn,
		error::DieselRepositoryError,
		models::account::{AccountDiesel, CreateAccountDiesel, UpdateAccountDiesel},
	},
};
pub struct AccountDieselRepository {
	pub pool: Arc<DBConn>,
}

impl AccountDieselRepository {
	pub fn new(db: Arc<DBConn>) -> Self {
		AccountDieselRepository { pool: db }
	}
}

#[async_trait]
impl AccountRepository for AccountDieselRepository {
	async fn create(&self, account: &CreateAccount) -> RepositoryResult<Account> {
		use crate::infrastructure::schema::accounts::dsl::accounts;
		let new_account_diesel: CreateAccountDiesel = CreateAccountDiesel::from(account.clone());
		let mut conn = self.pool.get().unwrap();
		let result = diesel::insert_into(accounts)
			.values(new_account_diesel)
			.get_result::<AccountDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(result.into())
	}

	async fn read(&self, account_num: &str) -> RepositoryResult<Account> {
		use crate::infrastructure::schema::accounts::dsl::{account_number, accounts};
		let mut conn = self.pool.get().unwrap();
		let result = accounts.filter(account_number.eq(account_num)).first::<AccountDiesel>(&mut conn);

		match result {
			Ok(value) => Ok(value.into()), // Extract value and convert if needed
			Err(_error) => Err(RepositoryError { message: "Result not found".to_string() }),
		}
	}

	async fn update(&self, account_num: &str, account: &UpdateAccount) -> RepositoryResult<Account> {
		use crate::infrastructure::schema::accounts::dsl::{account_number, accounts};
		let update_account_diesel = UpdateAccountDiesel::from(account.clone());
		let mut conn = self.pool.get().unwrap();
		let is_delete = accounts.filter(account_number.eq(account_num)).select(is_deleted).get_result::<bool>(&mut conn);
		if !is_delete.expect("is_deleted column not found") {
			let updated_account = diesel::update(accounts.filter(account_number.eq(account_num)))
				.set(&update_account_diesel)
				.get_result::<AccountDiesel>(&mut conn)
				.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
			Ok(updated_account.into())
		} else {
			return Err(RepositoryError { message: "Account is already deleted".to_string() });
		}
	}

	async fn delete(&self, account_num: &str, account: DeleteAccount) -> RepositoryResult<Account> {
		use crate::infrastructure::schema::accounts::dsl::{account_number, accounts};
		let delete_account_diesel = DeleteAccountDiesel::from(account.clone());
		let mut conn = self.pool.get().unwrap();
		let is_delete = accounts.filter(account_number.eq(account_num)).select(is_deleted).get_result::<bool>(&mut conn);
		if !is_delete.expect("is_deleted column not found") {
			let updated_account = diesel::update(accounts.filter(account_number.eq(account_num)))
				.set(&delete_account_diesel)
				.get_result::<AccountDiesel>(&mut conn);
			match updated_account {
				Ok(value) => Ok(value.into()), // Extract value and convert if needed
				Err(_error) => Err(RepositoryError { message: "Result not found".to_string() }),
			}
		} else {
			return Err(RepositoryError { message: "Account is already deleted".to_string() });
		}
	}
}
