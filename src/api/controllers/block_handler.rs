use crate::{
	api::dto::block::BlockDTO,
	domain::{
		error::ApiError,
		repositories::{block::BlockQueryParams, repository::ResultPaging},
		services::block::BlockService,
	},
};
use actix_web::{web, Result};

#[utoipa::path(
    post,
    path = "/api/blocks",
    tag = "Blocks",
    responses(
        (status = 200, description = "Block mined successfully", body = BlockDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn create_block(block_service: web::Data<dyn BlockService>) -> Result<web::Json<BlockDTO>, ApiError> {
	let block = block_service.create().await?;
	Ok(web::Json(block.into()))
}

#[utoipa::path(
    get,
    path = "/api/blocks",
    tag = "Blocks",
    responses(
        (status = 200, description = "Blocks retrieved successfully", body = [BlockDTO]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn list_blocks(
	block_service: web::Data<dyn BlockService>, params: web::Query<BlockQueryParams>,
) -> Result<web::Json<ResultPaging<BlockDTO>>, ApiError> {
	let selection = block_service.list(params.into_inner()).await?;
	Ok(web::Json(selection.into()))
}

#[utoipa::path(
    get,
    path = "/api/blocks/{number}",
    tag = "Blocks",
    params(
        ("number", description = "Block Height")
    ),
    responses(
        (status = 200, description = "Block found successfully", body = BlockDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn read_block(block_service: web::Data<dyn BlockService>, params: web::Path<i32>) -> Result<web::Json<BlockDTO>, ApiError> {
	let block = block_service.read(params.into_inner()).await?;
	Ok(web::Json(block.into()))
}
