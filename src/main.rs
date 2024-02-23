use dotenv::dotenv;
use ironclad::{
	domain::constants::{DB_HOST, DB_NAME, DB_PASSWORD, DB_USERNAME},
	infrastructure::databases::pgsql::db_pool,
	run,
};
use std::{env, net::TcpListener};
#[cfg(test)]
mod tests;

use job_scheduler::{Job, JobScheduler};
use reqwest::Client;
use std::{thread, time::Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	let address = TcpListener::bind("127.0.0.1:8000")?;
	let database_url = format!(
		"postgresql://{}:{}@{}:5432/{}",
		env::var(DB_USERNAME).expect("Username required"),
		env::var(DB_PASSWORD).expect("Password required"),
		env::var(DB_HOST).expect("Database Host required"),
		env::var(DB_NAME).expect("Database name required")
	);

	// Spawn a new thread
	thread::spawn(|| {
		post_request();
	});

	run(address, db_pool(&database_url))?.await
}

//run the asynchronous function which post the request in every 1 min
fn post_request() {
	let client = Client::new();
	let rt = tokio::runtime::Runtime::new().unwrap();
	let mut sched = JobScheduler::new();
	sched.add(Job::new("0 1/5 * * * *".parse().unwrap(), || {
		rt.block_on(async {
			let _response = client.post("http://127.0.0.1:8000/api/blocks").send().await;
		})
	}));
	loop {
		sched.tick();
		std::thread::sleep(Duration::from_millis(300000));
	}
}
