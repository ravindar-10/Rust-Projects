use async_trait::async_trait;

use crate::domain::{
	error::CommonError,
	models::account::{Account, DeleteAccount},
};

#[async_trait]
pub trait AccountService: Sync + Send
{
	async fn read(&self, account_number: &str) -> Result<Account, CommonError>;
	async fn delete(&self, account_number: &str, delete_account: DeleteAccount) -> Result<String, CommonError>;
}
