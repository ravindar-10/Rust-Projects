use async_trait::async_trait;
use uuid::Uuid;

use std::sync::Arc;

use crate::domain::{
	error::CommonError,
	models::{
		transaction::Transaction,
		user::{CreateUser, UpdateUser, User},
	},
	repositories::{
		repository::ResultPaging,
		user::{UserQueryParams, UserRepository},
	},
	services::{orchestrator::OrchestratorService, user::UserService},
};

pub struct UserServiceImpl
{
	pub repository: Arc<dyn UserRepository>,
	pub orchestrator_service: Arc<dyn OrchestratorService>,
}

impl UserServiceImpl
{
	pub fn new(repository: Arc<dyn UserRepository>, orchestrator_service: Arc<dyn OrchestratorService>) -> Self
	{
		UserServiceImpl { repository, orchestrator_service }
	}
}

#[async_trait]
impl UserService for UserServiceImpl
{
	async fn create(&self, user: CreateUser) -> Result<String, CommonError>
	{
		let uuid = Uuid::new_v4().to_string();
		let user = CreateUser { first_name: user.first_name, last_name: user.last_name, email: user.email, uuid: Some(uuid) };
		let txn: Transaction = self.orchestrator_service.register_user(user).await.map_err(|e| -> CommonError { e.into() })?;
		Ok(txn.transaction_hash)
	}

	async fn read(&self, id: i32) -> Result<User, CommonError>
	{
		self.repository.read(id).await.map_err(|e| -> CommonError { e.into() })
	}

	async fn read_by_email(&self, email: &str) -> Result<User, CommonError>
	{
		self.repository.read_by_email(email).await.map_err(|e| -> CommonError { e.into() })
	}

	async fn update(&self, email: &str, user: UpdateUser) -> Result<String, CommonError>
	{
		let cloned = user.clone();
		let txn: Transaction = self
			.orchestrator_service
			.update_user(email, cloned)
			.await
			.map_err(|e| -> CommonError { e.into() })?;
		Ok(txn.transaction_hash)
	}

	async fn delete(&self, email: &str) -> Result<(), CommonError>
	{
		self.repository.delete(email).await.map_err(|e| -> CommonError { e.into() })
	}

	async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, CommonError>
	{
		self.repository.list(params).await.map_err(|e| -> CommonError { e.into() })
	}
}
