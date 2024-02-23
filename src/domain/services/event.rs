use crate::domain::error::CommonError;
use crate::domain::models::event::CreateEvent;
use crate::domain::models::event::Event;
use async_trait::async_trait;

#[async_trait]
pub trait EventService: Sync + Send {
	async fn create(&self, event: CreateEvent) -> Result<Event, CommonError>;
}
