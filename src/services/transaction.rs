use async_trait::async_trait;

use std::sync::Arc;

use crate::{
	domain::{
		error::CommonError,
		models::transaction::{CreateTransaction, Transaction, UpdateTransaction},
		repositories::{
			repository::ResultPaging,
			transaction::{TransactionQueryParams, TransactionRepository},
		},
		services::{orchestrator::OrchestratorService, transaction::TransactionService},
	},
	services::validation::ValidateTransaction,
};

#[derive(Clone)]
pub struct TransactionServiceImpl {
	pub repository: Arc<dyn TransactionRepository>,
	pub orchestrator_service: Arc<dyn OrchestratorService>,
}

impl TransactionServiceImpl {
	pub fn new(repository: Arc<dyn TransactionRepository>, orchestrator_service: Arc<dyn OrchestratorService>) -> Self {
		TransactionServiceImpl { repository, orchestrator_service }
	}
}

#[async_trait]
impl TransactionService for TransactionServiceImpl {
	async fn create(&self, transaction: CreateTransaction) -> Result<Transaction, CommonError> {
		let mut cloned_txn = transaction.clone();
		cloned_txn.validate().map_err(|e| e.into())?; // validate the transaction
		let txn_hash_result = self
			.orchestrator_service
			.create_transaction_hash(transaction.clone())
			.await
			.map_err(|e| -> CommonError { e.into() });
		let txn_hash = txn_hash_result?;
		cloned_txn.transaction_hash = txn_hash;
		self.repository.create(&mut cloned_txn).await.map_err(|e| -> CommonError { e.into() })
	}

	async fn read(&self, transaction_hash: &str) -> Result<Transaction, CommonError> {
		self.repository.read(transaction_hash).await.map_err(|e| -> CommonError { e.into() })
	}

	async fn update(&self, _transaction_hash: &str, _txn: UpdateTransaction) -> Result<Transaction, CommonError> {
		todo!()
	}

	async fn list(&self, params: TransactionQueryParams) -> Result<ResultPaging<Transaction>, CommonError> {
		self.repository.list(params).await.map_err(|e| -> CommonError { e.into() })
	}

	async fn execute(&self, block_number: i32, transaction: &Transaction) -> Result<Transaction, CommonError> {
		self
			.orchestrator_service
			.execute_transaction(block_number, transaction.clone())
			.await
			.map_err(|e| -> CommonError { e.into() })
	}
}
