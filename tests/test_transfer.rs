mod common;
use actix_web::web;
use ironclad::{
	api::dto::{transaction::CreateTransactionDTO, user::CreateUserDTO},
	domain::models::{
		account::Account,
		transaction::{deposit::DepositTransaction, transfer::TransferTransaction},
	},
};

#[tokio::test]
async fn user_handler_works()
{
	let address = common::spawn_app().await;
	let client = reqwest::Client::new();

	//create user (sender)
	let sender_data = web::Json(CreateUserDTO {
		email: "sender@example.com".to_string(),
		first_name: "Sender First Name".to_string(),
		last_name: "Sender Last Name".to_string(),
	});

	let response = client
		.post(&format!("{}/api/users", &address))
		.json(&sender_data)
		.send()
		.await
		.expect("Failed to execute request.");
	assert_eq!(200, response.status().as_u16());
	//create user (receiver)
	let receiver_data = web::Json(CreateUserDTO {
		email: "reciever@example.com".to_string(),
		first_name: "receiver First Name".to_string(),
		last_name: "receiver Last Name".to_string(),
	});

	let response = client
		.post(&format!("{}/api/users", &address))
		.json(&receiver_data)
		.send()
		.await
		.expect("Failed to execute request.");
	assert_eq!(200, response.status().as_u16());

	//create block
	let response = client.post(&format!("{}/api/blocks", &address)).send().await.expect("Failed to execute request.");
	assert_eq!(200, response.status().as_u16());

	//create transaction to deposit amount in sender's account
	let deposit_data = serde_json::to_value(DepositTransaction { to_account_number: "IRONCLAD0000000002".to_string(), amount: 1000.0 }).unwrap();

	let transaction_data =
		web::Json(CreateTransactionDTO { user_id: 2, transaction_type_str: "Deposit".to_string(), transaction_data: deposit_data, uuid: None });

	let response = client
		.post(&format!("{}/api/transactions", &address))
		.json(&transaction_data)
		.send()
		.await
		.expect("Failed to execute request.");
	assert_eq!(200, response.status().as_u16());
	//create block
	let response = client.post(&format!("{}/api/blocks", &address)).send().await.expect("Failed to execute request.");
	assert_eq!(200, response.status().as_u16());
	//create transaction to transfer from one account to another
	let transfer_data = serde_json::to_value(TransferTransaction {
		from_account_number: "IRONCLAD0000000002".to_string(),
		to_account_number: "IRONCLAD0000000003".to_string(),
		amount: 1000.0,
	})
	.unwrap();

	let post_data = web::Json(CreateTransactionDTO {
		user_id: 1,
		transaction_type_str: "Transfer".to_string(),
		transaction_data: transfer_data,
		uuid: None,
	});

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
	//Sender's amount verification
	let response = client
		.get(&format!("{}/api/accounts/IRONCLAD0000000002", &address))
		.send()
		.await
		.expect("Failed to execute request.");
	let body = response.text().await.expect("Failed to read response body");
	let account_data: Account = serde_json::from_str(&body).expect("Failed to parse JSON");
	let balance = account_data.balance;
	assert_eq!(0.0, balance);
	//Receiver's amount verification
	let response = client
		.get(&format!("{}/api/accounts/IRONCLAD0000000003", &address))
		.send()
		.await
		.expect("Failed to execute request.");
	let body = response.text().await.expect("Failed to read response body");
	let account_data: Account = serde_json::from_str(&body).expect("Failed to parse JSON");
	let balance = account_data.balance;
	assert_eq!(1000.0, balance);
}
