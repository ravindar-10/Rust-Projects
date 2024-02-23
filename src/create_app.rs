use actix_web::{
	body::MessageBody,
	dev::{ServiceFactory, ServiceRequest, ServiceResponse},
	middleware::Logger,
	web, App, Error,
};
use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::api::controllers::{account_handler::delete_account, event_handler::create_event};
use crate::{
	api::controllers::{
		account_handler::read_account,
		block_handler::{create_block, list_blocks, read_block},
		health_check::health_check,
		transaction_handler::{create_transaction, list_transactions, read_transaction},
		user_handler::{create_user, delete_user, list_users, read_user, update_user},
	},
	container::Container,
	open_api::ApiDoc,
};

pub fn create_app(
	db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
) -> App<impl ServiceFactory<ServiceRequest, Response = ServiceResponse<impl MessageBody>, Config = (), InitError = (), Error = Error>> {
	let container = Container::new(db_pool);
	let openapi = ApiDoc::openapi();
	let user_service = container.user_service.clone();
	let block_service = container.block_service.clone();
	let txn_service = container.txn_service.clone();
	let account_service = container.account_service.clone();
	let orchestrator_service = container.orchestrator_service.clone();
	let event_service = container.event_service.clone();
	App::new()
		.app_data(web::Data::from(user_service.clone()))
		.app_data(web::Data::from(block_service.clone()))
		.app_data(web::Data::from(orchestrator_service.clone()))
		.app_data(web::Data::from(account_service.clone()))
		.app_data(web::Data::from(txn_service.clone()))
		.app_data(web::Data::from(event_service.clone()))
		.service(
			web::scope("/api/users")
				.route("", web::post().to(create_user))
				.route("", web::get().to(list_users))
				.route("/{email}", web::get().to(read_user))
				.route("/{email}", web::put().to(update_user))
				.route("/{email}", web::delete().to(delete_user)),
		)
		.service(
			web::scope("/api/blocks")
				.route("", web::post().to(create_block))
				.route("", web::get().to(list_blocks))
				.route("/{number}", web::get().to(read_block)),
		)
		.service(
			web::scope("/api/transactions")
				.route("", web::post().to(create_transaction))
				.route("", web::get().to(list_transactions))
				.route("/{txn_hash}", web::get().to(read_transaction)),
		)
		.service(
			web::scope("/api/accounts")
				.route("/{acc_number}", web::get().to(read_account))
				.route("/{acc_number}", web::put().to(delete_account)),
		)
		.service(web::scope("/api/event").route("", web::post().to(create_event)))
		.service(Redoc::with_url("/redoc", openapi.clone()))
		.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
		.service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
		.route("/health_check", web::get().to(health_check))
		.wrap(Logger::default())
}
