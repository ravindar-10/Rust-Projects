use crate::{
	api::dto::account::{AccountDTO, DeleteAccountDTO},
	domain::{error::ApiError, services::account::AccountService},
};
use actix_web::{web, Result};

#[utoipa::path(
    get,
    path = "/api/accounts/{acc_number}",
    tag = "Accounts",
    params(
        ("acc_number", description = "User's account number"),
    ),
    responses(
        (status = 200, description = "Account read successfully", body = AccountDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]

pub async fn read_account(account_service: web::Data<dyn AccountService>, acc_number: web::Path<String>) -> Result<web::Json<AccountDTO>, ApiError> {
	let account = account_service.read(&acc_number.into_inner()).await?;
	Ok(web::Json(account.into()))
}

#[utoipa::path(
    put,
    path = "/api/accounts/{acc_number}",
    tag = "Accounts",
    request_body = DeleteAccountDTO,
    params(
        ("acc_number", description = "User's account number"),
    ),
    responses(
        (status = 200, description = "Account deleted successfully", body = AccountDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]

pub async fn delete_account(
	account_service: web::Data<dyn AccountService>, acc_number: web::Path<String>, post_data: web::Json<DeleteAccountDTO>,
) -> Result<String, ApiError> {
	let account = account_service.delete(&acc_number.into_inner(), post_data.into_inner().into()).await?;
	Ok(account.into())
}
