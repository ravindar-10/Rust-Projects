use async_trait::async_trait;

use crate::domain::{
	error::CommonError,
	models::transaction::{CreateTransaction, Transaction, UpdateTransaction},
	repositories::{repository::ResultPaging, transaction::TransactionQueryParams},
};

#[async_trait]
pub trait TransactionService: Sync + Send
{
	async fn create(&self, transaction: CreateTransaction) -> Result<Transaction, CommonError>;
	async fn read(&self, transaction_hash: &str) -> Result<Transaction, CommonError>;
	async fn update(&self, transaction_hash: &str, txn: UpdateTransaction) -> Result<Transaction, CommonError>;
	async fn list(&self, params: TransactionQueryParams) -> Result<ResultPaging<Transaction>, CommonError>;
	async fn execute(&self, block_number: i32, transaction: &Transaction) -> Result<Transaction, CommonError>;
}
