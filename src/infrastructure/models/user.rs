use chrono;
use diesel::{AsChangeset, Insertable, Queryable};

use crate::{
	domain::models::user::{CreateUser, UpdateUser, User},
	infrastructure::schema::users,
};

#[derive(Queryable, Debug)]
#[diesel(table_name = users)]
pub struct UserDiesel
{
	pub user_id: i32,
	pub email: String,
	pub first_name: String,
	pub last_name: String,
	pub nonce: i32,
	pub registration_date: chrono::NaiveDateTime,
	pub uuid: String,
}

impl Into<User> for UserDiesel
{
	fn into(self) -> User
	{
		User {
			user_id: self.user_id,
			email: self.email,
			first_name: self.first_name,
			last_name: self.last_name,
			nonce: self.nonce,
			registration_date: self.registration_date,
		}
	}
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDiesel
{
	pub email: String,
	pub first_name: String,
	pub last_name: String,
	pub uuid: String,
}

impl From<CreateUser> for CreateUserDiesel
{
	fn from(u: CreateUser) -> Self
	{
		CreateUserDiesel {
			email: u.email,
			first_name: u.first_name,
			last_name: u.last_name,
			uuid: match u.uuid
			{
				Some(uuid) => uuid,
				None => "NA".to_string(),
			},
		}
	}
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserDiesel
{
	pub first_name: Option<String>,
	pub last_name: Option<String>,
}

impl From<UpdateUser> for UpdateUserDiesel
{
	fn from(u: UpdateUser) -> Self
	{
		UpdateUserDiesel { first_name: u.first_name, last_name: u.last_name }
	}
}
