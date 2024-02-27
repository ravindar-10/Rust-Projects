use crate::{
	api::dto::event::EventDTO,
	domain::{error::ApiError, models::event::CreateEvent, services::event::EventService},
};
use actix_web::{web, Result};

#[utoipa::path(
    post,
    path = "/api/events",
    tag = "Events",
    responses(
        (status = 200, description = "Event Created successfully", body = CreateEventDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn create_event<CreateEventDTO>(
	event_service: web::Data<dyn EventService>, post_data: web::Json<CreateEventDTO>,
) -> Result<web::Json<EventDTO>, ApiError>
where
	CreateEvent: std::convert::From<CreateEventDTO>,
{
	let event = event_service.create(post_data.into_inner().into()).await?;
	Ok(web::Json(event.into()))
}
