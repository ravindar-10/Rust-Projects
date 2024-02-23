use async_trait::async_trait;

use crate::domain::{
	models::account::{Account, CreateAccount, DeleteAccount, UpdateAccount},
	repositories::repository::RepositoryResult,
};

#[async_trait]
pub trait AccountRepository: Send + Sync {
	async fn create(&self, account: &CreateAccount) -> RepositoryResult<Account>;
	async fn read(&self, account_number: &str) -> RepositoryResult<Account>;
	async fn update(&self, account_number: &str, account: &UpdateAccount) -> RepositoryResult<Account>;
	async fn delete(&self, account_number: &str, account: DeleteAccount) -> RepositoryResult<Account>;
}
