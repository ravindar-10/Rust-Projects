use async_trait::async_trait;

use crate::domain::{
	error::CommonError,
	models::block::Block,
	repositories::{block::BlockQueryParams, repository::ResultPaging},
};

#[async_trait]
pub trait BlockService: Sync + Send
{
	async fn create(&self) -> Result<Block, CommonError>;
	async fn read(&self, block_number: i32) -> Result<Block, CommonError>;
	async fn delete(&self, block_number: i32) -> Result<(), CommonError>;
	async fn list(&self, params: BlockQueryParams) -> Result<ResultPaging<Block>, CommonError>;
}
