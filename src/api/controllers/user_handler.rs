use crate::{
	api::dto::user::{CreateUserDTO, UpdateUserDTO, UserDTO},
	domain::{
		error::ApiError,
		repositories::{repository::ResultPaging, user::UserQueryParams},
		services::user::UserService,
	},
};
use actix_web::{web, HttpResponse, Result};

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "Users",
    request_body = CreateUserDTO,
    responses(
        (status = 200, description = "User created successfully", body = UserDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn create_user(user_service: web::Data<dyn UserService>, post_data: web::Json<CreateUserDTO>) -> Result<String, ApiError> {
	let user = user_service.create(post_data.into_inner().into()).await?;
	Ok(user.into())
}

#[utoipa::path(
    get,
    path = "/api/users/{email}",
    tag = "Users",
    params(
        ("email", description = "User's email"),
    ),
    responses(
        (status = 200, description = "User read successfully", body = UserDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn read_user(user_service: web::Data<dyn UserService>, email: web::Path<String>) -> Result<web::Json<UserDTO>, ApiError> {
	let user = user_service.read_by_email(&email.into_inner()).await?;
	Ok(web::Json(user.into()))
}

#[utoipa::path(
    put,
    path = "/api/users/{email}",
    tag = "Users",
    request_body = UpdateUserDTO,
    params(
        ("email", description = "User's email"),
    ),
    responses(
        (status = 200, description = "User read successfully", body = UserDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn update_user(
	user_service: web::Data<dyn UserService>, email: web::Path<String>, post_data: web::Json<UpdateUserDTO>,
) -> Result<String, ApiError> {
	let user = user_service.update(&email.into_inner(), post_data.into_inner().into()).await?;
	Ok(user.into())
}

#[utoipa::path(
    delete,
    path = "/api/users/{email}",
    tag = "Users",
    params(
        ("email", description = "User's email")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn delete_user(user_service: web::Data<dyn UserService>, params: web::Path<String>) -> Result<HttpResponse, ApiError> {
	user_service.delete(&params.into_inner()).await?;
	Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "Users",
    responses(
        (status = 200, description = "Users retrieved successfully", body = [UserDTO]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn list_users(
	user_service: web::Data<dyn UserService>, params: web::Query<UserQueryParams>,
) -> Result<web::Json<ResultPaging<UserDTO>>, ApiError> {
	let selection = user_service.list(params.into_inner()).await?;
	Ok(web::Json(selection.into()))
}
