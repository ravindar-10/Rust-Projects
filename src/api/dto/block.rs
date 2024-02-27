use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::{models::block::Block, repositories::repository::ResultPaging};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockDTO
{
	pub block_number: i32,
	pub block_hash: String,
	pub transaction_count: i32,
	pub created_at: chrono::NaiveDateTime,
}

impl Into<BlockDTO> for Block
{
	fn into(self) -> BlockDTO
	{
		BlockDTO {
			block_number: self.block_number,
			block_hash: self.block_hash,
			transaction_count: self.transaction_count,
			created_at: self.created_at,
		}
	}
}

impl Into<ResultPaging<BlockDTO>> for ResultPaging<Block>
{
	fn into(self) -> ResultPaging<BlockDTO>
	{
		ResultPaging { total: self.total, items: self.items.into_iter().map(|block| block.into()).collect() }
	}
}
