use async_trait::async_trait;

use crate::domain::{
	error::CommonError,
	models::user::{CreateUser, UpdateUser, User},
	repositories::{repository::ResultPaging, user::UserQueryParams},
};
#[async_trait]
pub trait UserService: Sync + Send {
	async fn create(&self, user: CreateUser) -> Result<String, CommonError>;
	async fn read(&self, id: i32) -> Result<User, CommonError>;
	async fn read_by_email(&self, email: &str) -> Result<User, CommonError>;
	async fn update(&self, email: &str, user: UpdateUser) -> Result<String, CommonError>;
	async fn delete(&self, email: &str) -> Result<(), CommonError>;
	async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, CommonError>;
}
