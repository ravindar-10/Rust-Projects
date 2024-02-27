use crate::domain::{
	error::CommonError,
	models::event::{CreateEvent, Event},
};
use async_trait::async_trait;

#[async_trait]
pub trait EventService: Sync + Send
{
	async fn create(&self, event: CreateEvent) -> Result<Event, CommonError>;
}
