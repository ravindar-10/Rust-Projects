use crate::domain::{
	models::event::{CreateEvent, Event},
	repositories::repository::RepositoryResult,
};
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository: Send + Sync
{
	async fn create(&self, eve: &CreateEvent) -> RepositoryResult<Event>;
}
