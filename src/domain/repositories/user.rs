use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
	models::user::{CreateUser, UpdateUser, User},
	repositories::repository::{QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQueryParams
{
	pub limit: Option<i64>,
	pub offset: Option<i64>,
}

impl QueryParams for UserQueryParams
{
	fn limit(&self) -> i64
	{
		self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
	}
	fn offset(&self) -> i64
	{
		self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
	}
}

#[async_trait]
pub trait UserRepository: Send + Sync
{
	async fn create(&self, user: &CreateUser) -> RepositoryResult<User>;
	async fn read(&self, id: i32) -> RepositoryResult<User>;
	async fn read_by_email(&self, email: &str) -> RepositoryResult<User>;
	async fn update(&self, user_email: &str, user: &UpdateUser) -> RepositoryResult<User>;
	async fn increment_nonce(&self, email: &str) -> RepositoryResult<()>;
	async fn delete(&self, email: &str) -> RepositoryResult<()>;
	async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>>;
}
