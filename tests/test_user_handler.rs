mod common;
use actix_web::web;
// use chrono::NaiveDateTime;
// use ironclad::api::dto::block::BlockDTO;
// use ironclad::api::dto::transaction::CreateTransactionDTO;
use ironclad::api::dto::user::CreateUserDTO;
// use ironclad::domain::models::transaction::transfer::TransferTransaction;

#[tokio::test]
async fn user_handler_works()
{
	let address = common::spawn_app().await;
	let client = reqwest::Client::new();

	//create user
	let post_data = web::Json(CreateUserDTO {
		email: "test@example.com".to_string(),
		first_name: "Test User".to_string(),
		last_name: "Unit".to_string(),
	});

	let response = client
		.post(&format!("{}/api/users", &address))
		.json(&post_data)
		.send()
		.await
		.expect("Failed to execute request.");

	assert_eq!(200, response.status().as_u16());

	//read by email
	let email: String = "test@example.com".to_string();
	let response = client
		.get(&format!("{}/api/users/{}", &address, &email))
		.send()
		.await
		.expect("Failed to execute request.");

	assert_eq!(400, response.status().as_u16());

	//create block
	let response = client.post(&format!("{}/api/blocks", &address)).send().await.expect("Failed to execute request.");

	assert_eq!(200, response.status().as_u16());

	//read by email
	let email: String = "test@example.com".to_string();
	let response = client
		.get(&format!("{}/api/users/{}", &address, &email))
		.send()
		.await
		.expect("Failed to execute request.");

	assert_eq!(200, response.status().as_u16());
}
