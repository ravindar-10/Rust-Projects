use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, Default)]
pub struct Block
{
	pub block_number: i32,
	pub block_hash: String,
	pub transaction_count: i32,
	pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CreateBlock
{
	pub block_hash: String,
	pub transaction_count: i32,
}
