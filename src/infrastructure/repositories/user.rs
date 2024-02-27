use async_trait::async_trait;
use diesel::{prelude::*, RunQueryDsl};

use std::sync::Arc;

use crate::{
	domain::{
		error::RepositoryError,
		models::user::{CreateUser, UpdateUser, User},
		repositories::{
			repository::{QueryParams, RepositoryResult, ResultPaging},
			user::{UserQueryParams, UserRepository},
		},
	},
	infrastructure::{
		databases::pgsql::DBConn,
		error::DieselRepositoryError,
		models::user::{CreateUserDiesel, UpdateUserDiesel, UserDiesel},
	},
};

pub struct UserDieselRepository
{
	pub pool: Arc<DBConn>,
}

impl UserDieselRepository
{
	pub fn new(db: Arc<DBConn>) -> Self
	{
		UserDieselRepository { pool: db }
	}
}

#[async_trait]
impl UserRepository for UserDieselRepository
{
	async fn create(&self, user: &CreateUser) -> RepositoryResult<User>
	{
		use crate::infrastructure::schema::users::dsl::*;
		let new_user_diesel: CreateUserDiesel = CreateUserDiesel::from(user.clone());
		let mut conn = self.pool.get().unwrap();
		let result: UserDiesel = diesel::insert_into(users)
			.values(new_user_diesel)
			.get_result(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(result.into())
	}

	async fn read(&self, id: i32) -> RepositoryResult<User>
	{
		use crate::infrastructure::schema::users::dsl::*;
		let mut conn = self.pool.get().unwrap();
		users
			.find(id)
			.first::<UserDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())
			.map(|v| -> User { v.into() })
	}

	async fn read_by_email(&self, user_email: &str) -> RepositoryResult<User>
	{
		use crate::infrastructure::schema::users::dsl::*;
		let mut conn = self.pool.get().unwrap();
		users
			.filter(email.eq(user_email))
			.first::<UserDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())
			.map(|v| -> User { v.into() })
	}

	async fn update(&self, user_email: &str, user: &UpdateUser) -> RepositoryResult<User>
	{
		use crate::infrastructure::schema::users::dsl::*;
		let mut conn = self.pool.get().unwrap();
		let update_user_diesel: UpdateUserDiesel = UpdateUserDiesel::from(user.clone());
		let result = diesel::update(users.filter(email.eq(user_email)))
			.set(&update_user_diesel)
			.get_result::<UserDiesel>(&mut conn);
		match result
		{
			Ok(result) => Ok(result.into()), // Extract value and convert if needed
			Err(_error) => Err(RepositoryError { message: "Result not found".to_string() }),
		}
	}

	async fn increment_nonce(&self, user_email: &str) -> RepositoryResult<()>
	{
		use crate::infrastructure::schema::users::dsl::*;
		let mut conn = self.pool.get().unwrap();
		diesel::update(users.filter(email.eq(user_email)))
			.set(nonce.eq(nonce + 1))
			.execute(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(())
	}

	async fn delete(&self, user_email: &str) -> RepositoryResult<()>
	{
		use crate::infrastructure::schema::users::dsl::{email, users};
		let mut conn = self.pool.get().unwrap();
		diesel::delete(users)
			.filter(email.eq(user_email))
			.execute(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(())
	}

	async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>>
	{
		use crate::infrastructure::schema::users::dsl::users;
		let mut conn = self.pool.get().unwrap();
		let builder = users.limit(params.limit()).offset(params.offset());
		let result = builder.load::<UserDiesel>(&mut conn).map_err(|e| DieselRepositoryError::from(e).into_inner())?;
		Ok(ResultPaging { total: result.len() as i64, items: result.into_iter().map(|v| v.into()).collect() })
	}
}
