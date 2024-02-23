mod common;

#[tokio::test]
async fn health_check_works() {
	let address = common::spawn_app().await;
	let client = reqwest::Client::new();
	let response = client
		// Use the returned application address
		.get(&format!("{}/health_check", &address))
		.send()
		.await
		.expect("Failed to execute request.");
	assert!(response.status().is_success());
}
