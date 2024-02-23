use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
	models::block::{Block, CreateBlock},
	repositories::repository::{QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockQueryParams {
	pub limit: Option<i64>,
	pub offset: Option<i64>,
}

impl QueryParams for BlockQueryParams {
	fn limit(&self) -> i64 {
		self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
	}
	fn offset(&self) -> i64 {
		self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
	}
}

#[async_trait]
pub trait BlockRepository: Send + Sync {
	async fn create(&self, block: &CreateBlock) -> RepositoryResult<Block>;
	async fn read(&self, block_number: i32) -> RepositoryResult<Block>;
	async fn delete(&self, block_number: i32) -> RepositoryResult<()>;
	async fn list(&self, params: BlockQueryParams) -> RepositoryResult<ResultPaging<Block>>;
}
