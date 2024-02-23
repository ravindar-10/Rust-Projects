use actix_web::dev::Server;
use actix_web::HttpServer;
use create_app::create_app;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use env_logger::Env;
use infrastructure::databases::pgsql::run_migration;
use std::net::TcpListener;
use std::sync::Arc;

pub mod api;
pub mod container;
pub mod create_app;
pub mod domain;
pub mod infrastructure;
pub mod open_api;
pub mod services;
pub mod utils;

pub fn run(listener: TcpListener, db_pool: Pool<ConnectionManager<PgConnection>>) -> Result<Server, std::io::Error> {
	env_logger::init_from_env(Env::default().default_filter_or("debug"));
	let mut connection = db_pool.get().expect("Should get connection from pool");
	run_migration(&mut connection);
	let server = HttpServer::new(move || create_app(Arc::new(db_pool.clone()))).listen(listener)?.run();
	Ok(server)
}
