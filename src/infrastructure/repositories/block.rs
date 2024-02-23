use async_trait::async_trait;
use diesel::prelude::*;

use std::sync::Arc;

use crate::{
	domain::{
		models::block::{Block, CreateBlock},
		repositories::{
			block::{BlockQueryParams, BlockRepository},
			repository::{QueryParams, RepositoryResult, ResultPaging},
		},
	},
	infrastructure::{
		databases::pgsql::DBConn,
		error::DieselRepositoryError,
		models::block::{BlockDiesel, CreateBlockDiesel},
	},
};

pub struct BlockDieselRepository {
	pub pool: Arc<DBConn>,
}

impl BlockDieselRepository {
	pub fn new(db: Arc<DBConn>) -> Self {
		BlockDieselRepository { pool: db }
	}
}

#[async_trait]
impl BlockRepository for BlockDieselRepository {
	async fn create(&self, new_block: &CreateBlock) -> RepositoryResult<Block> {
		use crate::infrastructure::schema::blocks::dsl::blocks;
		let new_block_diesel: CreateBlockDiesel = CreateBlockDiesel::from(new_block.clone());
		let mut conn = self.pool.get().unwrap();
		let result: BlockDiesel = diesel::insert_into(blocks)
			.values(new_block_diesel)
			.get_result(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(result.into())
	}
	async fn read(&self, block_nmb: i32) -> RepositoryResult<Block> {
		use crate::infrastructure::schema::blocks::dsl::{block_number, blocks};
		let mut conn = self.pool.get().unwrap();
		blocks
			.filter(block_number.eq(block_nmb))
			.first::<BlockDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())
			.map(|v| -> Block { v.into() })
	}

	async fn delete(&self, block_nmb: i32) -> RepositoryResult<()> {
		use crate::infrastructure::schema::blocks::dsl::{block_number, blocks};
		let mut conn = self.pool.get().unwrap();
		diesel::delete(blocks)
			.filter(block_number.eq(block_nmb))
			.execute(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(())
	}

	async fn list(&self, params: BlockQueryParams) -> RepositoryResult<ResultPaging<Block>> {
		use crate::infrastructure::schema::blocks::dsl::{blocks, created_at};
		let mut conn = self.pool.get().unwrap();
		let builder = blocks.limit(params.limit()).offset(params.offset()).order_by(created_at.desc());
		let result = builder.load::<BlockDiesel>(&mut conn).map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(ResultPaging { total: result.len() as i64, items: result.into_iter().map(|v| v.into()).collect() })
	}
}
