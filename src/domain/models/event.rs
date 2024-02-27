use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Debug)]
pub struct Event
{
	pub event_id: i32,
	pub transaction_id: Option<i32>,
	pub event_type: String,
	pub event_data: Option<Value>,
	pub created_at: Option<NaiveDateTime>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateEvent
{
	pub transaction_id: Option<i32>,
	pub event_type: String,
	pub event_data: Option<Value>,
}
