use crate::domain::models::event::{CreateEvent, Event};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventDTO
{
	pub event_id: i32,
	pub transaction_id: Option<i32>,
	pub event_type: String,
	pub event_data: Option<Value>,
	pub created_at: Option<NaiveDateTime>,
}

impl From<Event> for EventDTO
{
	fn from(event: Event) -> EventDTO
	{
		EventDTO {
			event_id: event.event_id,
			transaction_id: Some(event.transaction_id.expect("Unable to get transaction_id")),
			event_type: event.event_type,
			event_data: Some(event.event_data.into()),
			created_at: Some(event.created_at.expect("Unable to get time")),
		}
	}
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateEventDTO
{
	pub transaction_id: Option<i32>,
	pub event_type: String,
	pub event_data: Option<Value>,
}

impl Into<CreateEvent> for CreateEventDTO
{
	fn into(self) -> CreateEvent
	{
		CreateEvent {
			transaction_id: (self.transaction_id),
			event_type: self.event_type,
			event_data: Some(self.event_data.into()),
		}
	}
}
