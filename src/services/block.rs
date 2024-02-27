use async_trait::async_trait;

use std::sync::Arc;

use crate::domain::{
	error::CommonError,
	models::block::Block,
	repositories::{
		block::{BlockQueryParams, BlockRepository},
		repository::ResultPaging,
	},
	services::{block::BlockService, orchestrator::OrchestratorService},
};

#[derive(Clone)]
pub struct BlockServiceImpl
{
	pub repository: Arc<dyn BlockRepository>,
	pub orchestrator_service: Arc<dyn OrchestratorService>,
}

impl BlockServiceImpl
{
	pub fn new(repository: Arc<dyn BlockRepository>, orchestrator_service: Arc<dyn OrchestratorService>) -> Self
	{
		BlockServiceImpl { repository, orchestrator_service }
	}
}

#[async_trait]
impl BlockService for BlockServiceImpl
{
	async fn create(&self) -> Result<Block, CommonError>
	{
		self.orchestrator_service.mine_block().await.map_err(|e| -> CommonError { e.into() })
	}

	async fn read(&self, block_number: i32) -> Result<Block, CommonError>
	{
		self.repository.read(block_number).await.map_err(|e| -> CommonError { e.into() })
	}

	async fn delete(&self, block_number: i32) -> Result<(), CommonError>
	{
		self.repository.delete(block_number).await.map_err(|e| -> CommonError { e.into() })
	}
	async fn list(&self, params: BlockQueryParams) -> Result<ResultPaging<Block>, CommonError>
	{
		self.repository.list(params).await.map_err(|e| -> CommonError { e.into() })
	}
}
