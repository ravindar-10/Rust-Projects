use crate::{
	domain::models::event::{CreateEvent, Event},
	infrastructure::schema::events,
};
use diesel::{self, prelude::*};
use serde_json::Value;

#[derive(Queryable)]
pub struct EventDiesel {
	pub event_id: i32,
	pub transaction_id: Option<i32>,
	pub event_type: String,
	pub event_data: Option<Value>,
	pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<Event> for EventDiesel {
	fn from(t: Event) -> Self {
		EventDiesel {
			event_id: t.event_id,
			transaction_id: Some(t.transaction_id.expect("REASON")),
			event_type: t.event_type,
			event_data: Some(t.event_data.into()),
			created_at: Some(t.created_at.expect("REASON")),
		}
	}
}

impl Into<Event> for EventDiesel {
	fn into(self) -> Event {
		Event {
			event_id: self.event_id,
			transaction_id: Some(self.transaction_id.expect("REASON")),
			event_type: self.event_type,
			event_data: Some(self.event_data.into()),
			created_at: Some(self.created_at.expect("REASON")),
		}
	}
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct CreateEventDiesel {
	pub transaction_id: Option<i32>,
	pub event_type: String,
	pub event_data: Option<Value>,
}

impl From<CreateEvent> for CreateEventDiesel {
	fn from(t: CreateEvent) -> Self {
		CreateEventDiesel {
			transaction_id: Some(t.transaction_id.expect("error in fetching transaction_id")),
			event_type: t.event_type,
			event_data: t.event_data.into(),
		}
	}
}

impl Into<CreateEvent> for CreateEventDiesel {
	fn into(self) -> CreateEvent {
		CreateEvent {
			transaction_id: Some(self.transaction_id.expect("error in fetching transaction_id")),
			event_type: self.event_type,
			event_data: Some(self.event_data.into()),
		}
	}
}
