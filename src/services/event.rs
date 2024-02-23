use crate::domain::error::CommonError;
use crate::domain::models::event::CreateEvent;
use crate::domain::models::event::Event;
use crate::domain::repositories::event::EventRepository;
use crate::domain::services::event::EventService;
use crate::domain::services::orchestrator::OrchestratorService;
use async_trait::async_trait;
use std::sync::Arc;
pub struct EventServiceImpl {
	pub repository: Arc<dyn EventRepository>,
	pub orchestrator_service: Arc<dyn OrchestratorService>,
}

impl EventServiceImpl {
	pub fn new(repository: Arc<dyn EventRepository>, orchestrator_service: Arc<dyn OrchestratorService>) -> Self {
		EventServiceImpl { repository, orchestrator_service }
	}
}

#[async_trait]
impl EventService for EventServiceImpl {
	async fn create(&self, event: CreateEvent) -> Result<Event, CommonError> {
		let mut cloned = event.clone();
		let eve: Event = self.repository.create(&mut cloned).await.map_err(|e| -> CommonError { e.into() })?;
		Ok(eve)
	}
}
