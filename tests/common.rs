use std::{env, net::TcpListener};

use diesel::connection::SimpleConnection;
use dotenv::dotenv;
use ironclad::{
	domain::constants::{DB_HOST, DB_NAME, DB_PASSWORD, DB_USERNAME},
	infrastructure::databases::pgsql::db_pool,
	run,
};
use log::info;
use uuid::Uuid;

pub async fn spawn_app() -> String {
	dotenv().ok();
	// let port=8000;
	let listener = TcpListener::bind("127.0.0.1:0").expect("Should bind to a random port");
	// We retrieve the port assigned to us by the OS
	let port = listener.local_addr().unwrap().port();
	let database_url = database_url();
	let database_name = Uuid::new_v4().to_string();
	let db_pool_initial = db_pool(&database_url);
	let mut connection = db_pool_initial.get().expect("Should get connection from pool");
	connection
		.batch_execute(format!(r#"CREATE DATABASE "{}";"#, database_name).as_str())
		.expect("Database should be created");
	let database_url = database_url_with_custom_name(&database_name);
	let db_pool = db_pool(&database_url);
	let server = run(listener, db_pool).expect("Should bind to address");
	let _ = tokio::spawn(server);
	// We return the application address to the caller!
	format!("http://127.0.0.1:{}", port)
}

fn database_url() -> String {
	format!(
		"postgresql://{}:{}@{}:5432/{}",
		env::var(DB_USERNAME).expect("Username required"),
		env::var(DB_PASSWORD).expect("Password required"),
		env::var(DB_HOST).expect("Database Host required"),
		env::var(DB_NAME).expect("Database name required")
	)
}

fn database_url_with_custom_name(database_name: &str) -> String {
	format!(
		"postgresql://{}:{}@{}:5432/{}",
		env::var(DB_USERNAME).expect("Username required"),
		env::var(DB_PASSWORD).expect("Password required"),
		env::var(DB_HOST).expect("Database Host required"),
		database_name
	)
}
