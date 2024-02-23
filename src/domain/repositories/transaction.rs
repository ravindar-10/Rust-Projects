use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
	models::transaction::{CreateTransaction, Transaction, UpdateTransaction},
	repositories::repository::{QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionQueryParams {
	pub limit: Option<i64>,
	pub offset: Option<i64>,
	pub status: Option<String>,
}

impl QueryParams for TransactionQueryParams {
	fn limit(&self) -> i64 {
		self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
	}
	fn offset(&self) -> i64 {
		self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
	}
}

#[async_trait]
pub trait TransactionRepository: Send + Sync {
	async fn create(&self, txn: &CreateTransaction) -> RepositoryResult<Transaction>;
	async fn read(&self, txn_hash: &str) -> RepositoryResult<Transaction>;
	async fn update(&self, txn_hash: &str, txn: UpdateTransaction) -> RepositoryResult<Transaction>;
	async fn list(&self, params: TransactionQueryParams) -> RepositoryResult<ResultPaging<Transaction>>;
}
