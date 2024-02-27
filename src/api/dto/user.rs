use chrono;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::domain::{
	models::user::{CreateUser, UpdateUser, User},
	repositories::repository::ResultPaging,
};
#[derive(Debug, Clone, Validate, Serialize, Deserialize, ToSchema)]
pub struct UserDTO
{
	pub user_id: i32,
	pub email: String,
	pub first_name: String,
	pub last_name: String,
	pub registration_date: chrono::NaiveDateTime,
}

impl Into<UserDTO> for User
{
	fn into(self) -> UserDTO
	{
		UserDTO {
			user_id: self.user_id,
			email: self.email,
			first_name: self.first_name,
			last_name: self.last_name,
			registration_date: self.registration_date,
		}
	}
}

impl Into<ResultPaging<UserDTO>> for ResultPaging<User>
{
	fn into(self) -> ResultPaging<UserDTO>
	{
		ResultPaging { total: self.total, items: self.items.into_iter().map(|user| user.into()).collect() }
	}
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, ToSchema)]
pub struct CreateUserDTO
{
	#[validate(email)]
	pub email: String,
	#[validate(length(min = 1), length(max = 100))]
	pub first_name: String,
	#[validate(length(min = 1), length(max = 100))]
	pub last_name: String,
}

impl Into<CreateUser> for CreateUserDTO
{
	fn into(self) -> CreateUser
	{
		CreateUser { email: self.email, first_name: self.first_name, last_name: self.last_name, uuid: None }
	}
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserDTO
{
	pub email: String,
	#[validate(length(min = 1), length(max = 100))]
	pub first_name: Option<String>,
	#[validate(length(min = 1), length(max = 100))]
	pub last_name: Option<String>,
}

impl Into<UpdateUser> for UpdateUserDTO
{
	fn into(self) -> UpdateUser
	{
		UpdateUser { email: self.email, first_name: self.first_name, last_name: self.last_name }
	}
}
