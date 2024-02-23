mod common;
use actix_web::web;
// use chrono::NaiveDateTime;
// use ironclad::api::dto::block::BlockDTO;
// use ironclad::api::dto::transaction::CreateTransactionDTO;
use ironclad::api::dto::user::CreateUserDTO;
// use ironclad::domain::models::transaction::transfer::TransferTransaction;

#[tokio::test]
async fn user_handler_works() {
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

	//list users
	//  let response = client
	//  .get(&format!("{}/api/users", &address))
	//  .send()
	//  .await
	//  .expect("Failed to execute request.");

	//  assert!(response.status().is_success());

	//    //update user
	//    let email:String="test@example.com".to_string();

	//    let post_data = web::Json(CreateUserDTO {
	//     email: "test@example.com".to_string(),
	//     first_name: "Test User".to_string(),
	//     last_name:"Unit".to_string(),
	//     });

	//     let response = client
	// 		.put(&format!("{}/api/users/{}", &address,&email))
	//     .json(&post_data)
	// 		.send()
	// 		.await
	// 		.expect("Failed to execute request.");

	//    assert_eq!(400, response.status().as_u16());

	//    //delete user
	//    let email:String="test@example.com".to_string();

	//     let response = client
	// 		.delete(&format!("{}/api/users/{}", &address,&email))
	// 		.send()
	// 		.await
	// 		.expect("Failed to execute request.");

	//    assert_eq!(204, response.status().as_u16());

	//create block
	//   let parsed_datetime = NaiveDateTime::parse_from_str("2024-01-16 23:21:12", "%Y-%m-%d %H:%M:%S").unwrap();
	//   let post_data = web::Json(BlockDTO {
	//     block_number:1234,
	// 		block_hash: "sdjksdj".to_string(),
	// 		transaction_count: 2,
	// 		created_at: parsed_datetime
	// });

	//   //read block
	//   let response = client
	//   .get(&format!("{}/api/blocks/{}", &address,1))
	//   .send()
	//   .await
	//   .expect("Failed to execute request.");

	//  assert_eq!(200, response.status().as_u16());

	//  //list block
	//  let response = client
	//  .get(&format!("{}/api/blocks", &address))
	//  .send()
	//  .await
	//  .expect("Failed to execute request.");

	// assert_eq!(200, response.status().as_u16());

	// //create transaction
	// let transaction_data = serde_json::to_value(TransferTransaction {
	//   from_account_number:"2323234".to_string(),
	//   to_account_number:"5464".to_string(),
	//   amount:121.0
	// }).unwrap();

	// let post_data = web::Json(CreateTransactionDTO {
	//   user_id: 2,
	//   transaction_type_str:"Transfer".to_string(),
	//   transaction_data
	// });

	// let response = client
	//   .post(&format!("{}/api/transactions", &address))
	//   .json(&post_data)
	//   .send()
	//   .await
	//   .expect("Failed to execute request.");

	//  assert_eq!(200, response.status().as_u16());

	// //read Transaction
	// let trans_hash="1eb2ff3d052c247e1fc731574be01c6340e0c00c923c929c19b484474ae9d4e8";
	// let response = client
	// .get(&format!("{}/api/transactions/{}", &address,&trans_hash))
	// .send()
	// .await
	// .expect("Failed to execute request.");

	// assert_eq!(400, response.status().as_u16());

	// //read_account
	// let account_number="SBI12000";
	// let response = client
	// .get(&format!("{}/api/transactions/{}", &address,&account_number))
	// .send()
	// .await
	// .expect("Failed to execute request.");

	// assert_eq!(400, response.status().as_u16());

	//    //list Transaction
	//  let response = client
	//  .get(&format!("{}/api/transactions", &address))
	//  .send()
	//  .await
	//  .expect("Failed to execute request.");

	// assert_eq!(200, response.status().as_u16());
}
