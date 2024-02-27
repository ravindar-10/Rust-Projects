use crate::{
	api::dto::transaction::{CreateTransactionDTO, TransactionDTO},
	domain::{
		error::ApiError,
		repositories::{repository::ResultPaging, transaction::TransactionQueryParams},
		services::transaction::TransactionService,
	},
};
use actix_web::{web, Result};

#[utoipa::path(
	post,
	path = "/api/transactions",
	tag = "Transactions",
	request_body = CreateTransactionDTO,
	responses(
		(status = 200, description = "Transaction created successfully", body = CreateTransactionDTO),
		(status = 400, description = "Bad Request"),
		(status = 500, description = "Internal Server Error"),
		(status = 503, description = "Service Unavailable"),
		(status = 429, description = "Too Many Requests"),
	)
)]
pub async fn create_transaction(
	transaction_service: web::Data<dyn TransactionService>, post_data: web::Json<CreateTransactionDTO>,
) -> Result<web::Json<TransactionDTO>, ApiError>
{
	let transaction = transaction_service.create(post_data.into_inner().into()).await?;
	Ok(web::Json(transaction.into()))
}

#[utoipa::path(
	get,
	path = "/api/transactions",
	tag = "Transactions",
	responses(
		(status = 200, description = "List of Transactions returned successfully", body = [TransactionDTO]),
		(status = 400, description = "Bad Request"),
		(status = 500, description = "Internal Server Error"),
		(status = 503, description = "Service Unavailable"),
		(status = 429, description = "Too Many Requests"),
	)
)]
pub async fn list_transactions(
	transaction_service: web::Data<dyn TransactionService>, params: web::Query<TransactionQueryParams>,
) -> Result<web::Json<ResultPaging<TransactionDTO>>, ApiError>
{
	let selection = transaction_service.list(params.into_inner()).await?;
	Ok(web::Json(selection.into()))
}

#[utoipa::path(
	get,
	path = "/api/transactions/{txn_hash}",
	tag = "Transactions",
	params(
		("txn_hash", description = "Unique transaction hash")
	),
	responses(
		(status = 200, description = "Transaction found successfully", body = TransactionDTO),
		(status = 400, description = "Bad Request"),
		(status = 500, description = "Internal Server Error"),
		(status = 503, description = "Service Unavailable"),
		(status = 429, description = "Too Many Requests"),
	)
)]
pub async fn read_transaction(
	transaction_service: web::Data<dyn TransactionService>, params: web::Path<String>,
) -> Result<web::Json<TransactionDTO>, ApiError>
{
	let transaction = transaction_service.read(&params.into_inner()).await?;
	Ok(web::Json(transaction.into()))
}
