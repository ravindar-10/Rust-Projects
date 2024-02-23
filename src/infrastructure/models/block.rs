use crate::{
	domain::models::block::{Block, CreateBlock},
	infrastructure::schema::blocks,
};
use diesel::{self, prelude::*};

#[derive(Queryable)]
pub struct BlockDiesel {
	pub block_number: i32,
	pub block_hash: String,
	pub transaction_count: i32,
	pub created_at: chrono::NaiveDateTime,
}

impl From<Block> for BlockDiesel {
	fn from(t: Block) -> Self {
		BlockDiesel {
			block_number: t.block_number,
			block_hash: t.block_hash,
			transaction_count: t.transaction_count,
			created_at: t.created_at,
		}
	}
}

impl Into<Block> for BlockDiesel {
	fn into(self) -> Block {
		Block {
			block_number: self.block_number,
			block_hash: self.block_hash,
			transaction_count: self.transaction_count,
			created_at: self.created_at,
		}
	}
}

#[derive(Insertable)]
#[diesel(table_name = blocks)]
pub struct CreateBlockDiesel {
	pub block_hash: String,
	pub transaction_count: i32,
}

impl From<CreateBlock> for CreateBlockDiesel {
	fn from(t: CreateBlock) -> Self {
		CreateBlockDiesel { block_hash: t.block_hash, transaction_count: t.transaction_count }
	}
}

impl Into<CreateBlock> for CreateBlockDiesel {
	fn into(self) -> CreateBlock {
		CreateBlock { block_hash: self.block_hash, transaction_count: self.transaction_count }
	}
}
