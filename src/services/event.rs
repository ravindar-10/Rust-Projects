use crate::domain::{
	error::CommonError,
	models::event::{CreateEvent, Event},
	repositories::event::EventRepository,
	services::{event::EventService, orchestrator::OrchestratorService},
};
use async_trait::async_trait;
use std::sync::Arc;
pub struct EventServiceImpl
{
	pub repository: Arc<dyn EventRepository>,
	pub orchestrator_service: Arc<dyn OrchestratorService>,
}

impl EventServiceImpl
{
	pub fn new(repository: Arc<dyn EventRepository>, orchestrator_service: Arc<dyn OrchestratorService>) -> Self
	{
		EventServiceImpl { repository, orchestrator_service }
	}
}

#[async_trait]
impl EventService for EventServiceImpl
{
	async fn create(&self, event: CreateEvent) -> Result<Event, CommonError>
	{
		let mut cloned = event.clone();
		let eve: Event = self.repository.create(&mut cloned).await.map_err(|e| -> CommonError { e.into() })?;
		Ok(eve)
	}
}
