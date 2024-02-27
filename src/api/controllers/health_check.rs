use actix_web::HttpResponse;

#[utoipa::path(
    get,
    path = "/health_check",
    tag = "Ironclad",
    responses(
        (status = 200, description = "Application is up"),
        (status = 503, description = "Service Unavailable"),
    )
)]
pub async fn health_check() -> HttpResponse
{
	HttpResponse::Ok().finish()
}
