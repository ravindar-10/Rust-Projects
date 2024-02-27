use async_trait::async_trait;

use std::sync::Arc;

use crate::domain::{
	error::CommonError,
	models::{
		account::{Account, DeleteAccount},
		transaction::Transaction,
	},
	repositories::account::AccountRepository,
	services::{account::AccountService, orchestrator::OrchestratorService},
};

pub struct AccountServiceImpl
{
	pub repository: Arc<dyn AccountRepository>,
	pub orchestrator_service: Arc<dyn OrchestratorService>,
}

impl AccountServiceImpl
{
	pub fn new(repository: Arc<dyn AccountRepository>, orchestrator_service: Arc<dyn OrchestratorService>) -> Self
	{
		AccountServiceImpl { repository, orchestrator_service }
	}
}

#[async_trait]
impl AccountService for AccountServiceImpl
{
	async fn read(&self, account_number: &str) -> Result<Account, CommonError>
	{
		self.repository.read(account_number).await.map_err(|e| -> CommonError { e.into() })
	}
	async fn delete(&self, _account_number: &str, delete_account: DeleteAccount) -> Result<String, CommonError>
	{
		let cloned = delete_account.clone();
		let txn: Transaction = self.orchestrator_service.delete_account(cloned).await.map_err(|e| -> CommonError { e.into() })?;
		Ok(txn.transaction_hash)
	}
}
