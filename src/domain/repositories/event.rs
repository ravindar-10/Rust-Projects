use crate::domain::models::event::CreateEvent;
use crate::domain::models::event::Event;
use crate::domain::repositories::repository::RepositoryResult;
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository: Send + Sync {
	async fn create(&self, eve: &CreateEvent) -> RepositoryResult<Event>;
}
