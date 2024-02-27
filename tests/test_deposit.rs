mod common;
use actix_web::web;
use ironclad::{
	api::dto::{transaction::CreateTransactionDTO, user::CreateUserDTO},
	domain::models::{account::Account, transaction::deposit::DepositTransaction},
};

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
	//create block
	let response = client.post(&format!("{}/api/blocks", &address)).send().await.expect("Failed to execute request.");
	assert_eq!(200, response.status().as_u16());
	//create transaction for deposit
	let deposit_data = serde_json::to_value(DepositTransaction { to_account_number: "IRONCLAD0000000002".to_string(), amount: 121.0 }).unwrap();

	let post_data =
		web::Json(CreateTransactionDTO { user_id: 2, transaction_type_str: "Deposit".to_string(), transaction_data: deposit_data, uuid: None });

	let response = client
		.post(&format!("{}/api/transactions", &address))
		.json(&post_data)
		.send()
		.await
		.expect("Failed to execute request.");

	assert_eq!(200, response.status().as_u16());
	//create block
	let response = client.post(&format!("{}/api/blocks", &address)).send().await.expect("Failed to execute request.");
	assert_eq!(200, response.status().as_u16());
	//get account details
	let response = client
		.get(&format!("{}/api/accounts/IRONCLAD0000000002", &address))
		.send()
		.await
		.expect("Failed to execute request.");
	let body = response.text().await.expect("Failed to read response body");
	let account_data: Account = serde_json::from_str(&body).expect("Failed to parse JSON");
	let balance = account_data.balance;
	assert_eq!(balance, 121.0);
}
