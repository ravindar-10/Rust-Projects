use crate::{
	domain::{
		models::event::{CreateEvent, Event},
		repositories::{event::EventRepository, repository::RepositoryResult},
	},
	infrastructure::{
		databases::pgsql::DBConn,
		error::DieselRepositoryError,
		models::event::{CreateEventDiesel, EventDiesel},
		schema::events::dsl::events,
	},
};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
pub struct EventDieselRepository
{
	pub pool: Arc<DBConn>,
}

impl EventDieselRepository
{
	pub fn new(db: Arc<DBConn>) -> Self
	{
		EventDieselRepository { pool: db }
	}
}

#[async_trait]
impl EventRepository for EventDieselRepository
{
	async fn create(&self, new_event: &CreateEvent) -> RepositoryResult<Event>
	{
		let new_event_diesel: CreateEventDiesel = CreateEventDiesel::from(new_event.clone());
		let mut conn = self.pool.get().unwrap();
		let result: EventDiesel = diesel::insert_into(events)
			.values(new_event_diesel)
			.get_result(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(result.into())
	}
}
