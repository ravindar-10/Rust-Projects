use async_trait::async_trait;

use crate::domain::{
	error::CommonError,
	models::{
		account::DeleteAccount,
		block::Block,
		transaction::{CreateTransaction, Transaction},
		user::{CreateUser, UpdateUser},
	},
};

#[async_trait]
pub trait OrchestratorService: Sync + Send
{
	async fn mine_block(&self) -> Result<Block, CommonError>;
	async fn register_user(&self, user: CreateUser) -> Result<Transaction, CommonError>;
	async fn create_transaction_hash(&self, transaction: CreateTransaction) -> Result<String, CommonError>;
	async fn execute_transaction(&self, block_number: i32, transaction: Transaction) -> Result<Transaction, CommonError>;
	async fn update_user(&self, email: &str, user: UpdateUser) -> Result<Transaction, CommonError>;
	async fn delete_account(&self, delete_account: DeleteAccount) -> Result<Transaction, CommonError>;
}
