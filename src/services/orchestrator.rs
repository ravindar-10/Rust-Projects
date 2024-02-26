use crate::domain::models::account::DeleteAccount;
use crate::domain::models::user::{UpdateUser};
use crate::domain::models::{account::Account, event::CreateEvent};
use crate::domain::repositories::event::EventRepository;
use crate::{
	domain::{
		constants::TRANSACTIONS_PER_BLOCK,
		error::CommonError,
		models::{
			account::{CreateAccount, UpdateAccount},
			block::{Block, CreateBlock},
			transaction::{
				deposit::DepositTransaction, status::TransactionStatus, transfer::TransferTransaction, txn_type::TransactionType,
				withdrawal::WithdrawalTransaction, CreateTransaction, Transaction, UpdateTransaction,
			},
			user::{CreateUser, User},
		},
		repositories::{
			account::AccountRepository,
			block::BlockRepository,
			transaction::{TransactionQueryParams, TransactionRepository},
			user::UserRepository,
		},
		services::orchestrator::OrchestratorService,
	},
	utils::{
		account_utils::generate_account_number,
		hash_util::{generate_hash, generate_hash_from_vector},
	},
};
use actix_web::Result;
use async_trait::async_trait;
use log::info;
use serde_json::{from_value, json, Value};
use std::{env, sync::Arc};
#[derive(Clone)]
pub struct OrchestratorServiceImpl {
	pub user_repository: Arc<dyn UserRepository>,
	pub account_repository: Arc<dyn AccountRepository>,
	pub txn_repository: Arc<dyn TransactionRepository>,
	pub block_repository: Arc<dyn BlockRepository>,
	pub event_repository: Arc<dyn EventRepository>,
}

impl OrchestratorServiceImpl {
	pub fn new(
		user_repository: Arc<dyn UserRepository>, account_repository: Arc<dyn AccountRepository>, txn_repository: Arc<dyn TransactionRepository>,
		block_repository: Arc<dyn BlockRepository>, event_repository: Arc<dyn EventRepository>,
	) -> Self {
		OrchestratorServiceImpl { user_repository, account_repository, txn_repository, block_repository, event_repository }
	}

	/**
	 * Perform deposit operation
	 * @param deposit_txn DepositTransaction
	 * @param txn_hash String
	 * @return bool
	 */
	async fn deposit(&self, deposit_txn: DepositTransaction, txn_hash: String) -> Result<bool, CommonError> {
		let to_account_number: &str = &deposit_txn.to_account_number;
		let amount: f64 = deposit_txn.amount;
		let recipient_account_result = self.account_repository.read(to_account_number).await.map_err(|e| -> CommonError { e.into() });

		let recipient_account: Account = match recipient_account_result {
			Ok(recipient_account) => recipient_account, // Extract value and convert if needed
			Err(_error) => return Ok(false),
		};
		let updated_account = UpdateAccount { latest_transaction_hash: txn_hash.clone(), balance: Some(recipient_account.balance + amount) };
		let update_result = self
			.account_repository
			.update(to_account_number, &updated_account)
			.await
			.map_err(|e| -> CommonError { e.into() });
		match update_result {
			Ok(_recipient_account) => Ok(true), // Extract value and convert if needed
			Err(_error) => return Ok(false),
		}
	}

	/**
	 * Perform withdrawal operation
	 * @param withdrawal_txn WithdrawalTransaction
	 * @param txn_hash String
	 * @return bool
	 */
	async fn withdrawal(&self, withdrawal_txn: WithdrawalTransaction, txn_hash: String) -> Result<bool, CommonError> {
		let from_account_number: &str = &withdrawal_txn.from_account_number;
		let amount: f64 = withdrawal_txn.amount;
		let source_account_result = self.account_repository.read(from_account_number).await.map_err(|e| -> CommonError { e.into() });
		let source_account: Account = match source_account_result {
			Ok(source_account) => source_account, // Extract value and convert if needed
			Err(_error) => return Ok(false),
		};
		// if from_account_number==source_account.account_number{
		// 	return Ok(false)
		// }
		// Check if source account has sufficient balance
		if source_account.balance >= amount {
			let updated_account = UpdateAccount { latest_transaction_hash: txn_hash.clone(), balance: Some(source_account.balance - amount) };
			let update_result = self
				.account_repository
				.update(from_account_number, &updated_account)
				.await
				.map_err(|e| -> CommonError { e.into() });
			match update_result {
				Ok(_recipient_account) => Ok(true), // Extract value and convert if needed
				Err(_error) => return Ok(false),
			}
		} else {
			Ok(false)
		}
	}
}

#[async_trait]
impl OrchestratorService for OrchestratorServiceImpl {
	/**
	 * Mines a block and executes the transactions within the block.
	 * @return Block
	 */
	async fn mine_block(&self) -> Result<Block, CommonError> {
		let transactions_per_block: i32 = env::var(TRANSACTIONS_PER_BLOCK)
			.expect(&*format!("{value} must be set", value = TRANSACTIONS_PER_BLOCK))
			.parse()
			.expect("Failed to parse pool size per worker as u32");
		let transaction_query_params = TransactionQueryParams {
			limit: Some(transactions_per_block as i64),
			offset: Some(0),
			status: Some(TransactionStatus::PENDING.to_string()),
		};
		let pending_transactions = self.txn_repository.list(transaction_query_params).await.map_err(|e| -> CommonError { e.into() })?;
		if pending_transactions.items.is_empty() {
			return Ok(Default::default()); // Or any other appropriate value for an empty block
		}
		let mut transaction_hashes: Vec<String> = Vec::new();
		for transaction in &pending_transactions.items {
			transaction_hashes.push(transaction.transaction_hash.clone());
		}
		let transaction_hashes_string = generate_hash_from_vector(&transaction_hashes);
		let mut new_block = CreateBlock {
			block_hash: transaction_hashes_string.clone(),
			transaction_count: pending_transactions.items.len() as i32,
		};
		let created_block = self.block_repository.create(&mut new_block).await.map_err(|e| -> CommonError { e.into() })?;
		// Execute the transactions
		for txn in pending_transactions.items {
			self.execute_transaction(created_block.block_number, txn.clone()).await?;
		}
		Ok(created_block)
	}

	/**
	 * Registers a transaction for User creation.
	 * @param user
	 * @return Transaction
	 */
	async fn register_user(&self, user: CreateUser) -> Result<Transaction, CommonError> {
		let json_data = serde_json::to_value(&user).unwrap();
		let txn = CreateTransaction {
			user_id: 1,                                     // Root user's id
			transaction_type: TransactionType::AccountOpen, // txn type for Account_Open
			transaction_data: json_data.clone(),
			transaction_hash: generate_hash(&json_data), // one way sha256 hash
		};
		self.txn_repository.create(&txn).await.map_err(|e| -> CommonError { e.into() })
	}

	/**
	 * Registers a transaction for Update user creation.
	 * @param user
	 * @return Transaction
	 */
	async fn update_user(&self, _email: &str, user: UpdateUser) -> Result<Transaction, CommonError> {
		let json_data = serde_json::to_value(&user).unwrap();
		let txn = CreateTransaction {
			user_id: 1,                                       // Root user's id
			transaction_type: TransactionType::AccountUpdate, // txn type for Account_Open
			transaction_data: json_data.clone(),
			transaction_hash: generate_hash(&json_data), // one way sha256 hash
		};
		self.txn_repository.create(&txn).await.map_err(|e| -> CommonError { e.into() })
	}

	/**
	 * Registers a transaction for Delete Account.
	 * @param user
	 * @return Transaction
	 */
	async fn delete_account(&self, delete_account: DeleteAccount) -> Result<Transaction, CommonError> {
		let json_data = serde_json::to_value(&delete_account).unwrap();
		let txn = CreateTransaction {
			user_id: 1,                                      // Root user's id
			transaction_type: TransactionType::AccountClose, // txn type for Account_Close
			transaction_data: json_data.clone(),
			transaction_hash: generate_hash(&json_data), // one way sha256 hash
		};
		self.txn_repository.create(&txn).await.map_err(|e| -> CommonError { e.into() })
	}
	/**
	 * Creates a unique hash for the transaction.
	 * @param transaction
	 * @return String
	 */
	async fn create_transaction_hash(&self, transaction: CreateTransaction) -> Result<String, CommonError> {
		// First find the latest nonce for the user and increment it
		let user_id = transaction.user_id;
		let user = self.user_repository.read(user_id).await.map_err(|e| -> CommonError { e.into() })?;
		let nonce = user.nonce + 1;
		self.user_repository.increment_nonce(&user.email).await.map_err(|e| -> CommonError { e.into() })?; // increment nonce for the user (for next transaction

		// Create a JSON object with additional fields (nonce and user_id)
		let additional_fields = json!({
				"nonce": nonce,
				"user_id": user_id,
		});

		// Merge the additional fields with the original JSON data
		let json_data = serde_json::to_value(&transaction.transaction_data).unwrap();
		let mut merged_json_data = json_data.as_object().cloned().unwrap_or_default();

		for (key, value) in additional_fields.as_object().unwrap().iter() {
			merged_json_data.insert(key.to_string(), value.clone());
		}

		// Generate the hash of the merged JSON data, this makes it unique
		let value: Value = merged_json_data.into();
		Ok(generate_hash(&value))
	}

	/**
	 * Executes a transaction.
	 * @param block_number
	 * @param transaction
	 * @return Transaction
	 */
	async fn execute_transaction(&self, block_number: i32, transaction: Transaction) -> Result<Transaction, CommonError> {
		let mut txn_status = TransactionStatus::FAILED;
		let txn_hash = transaction.transaction_hash.clone();
		let transaction_type = transaction.transaction_type;
		let data = &transaction.transaction_data;
		if let TransactionType::Deposit = transaction_type
		//Deposit
		{
			let deposit_txn: DepositTransaction = data.clone().into();
			let result: bool = self.deposit(deposit_txn, txn_hash.clone()).await.map_err(|e| -> CommonError { e.into() })?;
			if result {
				txn_status = TransactionStatus::SUCCESS;
			} else {
				txn_status = TransactionStatus::FAILED;
			}
		} else if let TransactionType::Withdrawal = transaction_type
		//Withdrawal
		{
			let withdrawal_txn: WithdrawalTransaction = data.clone().into();
			let result: bool = self.withdrawal(withdrawal_txn, txn_hash.clone()).await.map_err(|e| -> CommonError { e.into() })?;
			if result {
				txn_status = TransactionStatus::SUCCESS;
			} else {
				txn_status = TransactionStatus::FAILED;
			}
		} else if let TransactionType::Transfer = transaction_type
		//Transfer
		{
			let transfer_txn: TransferTransaction = data.clone().into();
			let withdrawal_txn: WithdrawalTransaction =
				WithdrawalTransaction { from_account_number: transfer_txn.from_account_number.clone(), amount: transfer_txn.amount };
			let deposit_txn: DepositTransaction =
				DepositTransaction { to_account_number: transfer_txn.to_account_number.clone(), amount: transfer_txn.amount };
			let withdrawal_result: bool = self.withdrawal(withdrawal_txn, txn_hash.clone()).await.map_err(|e| -> CommonError { e.into() })?;
			if withdrawal_result {
				let deposit_result: bool = self.deposit(deposit_txn, txn_hash.clone()).await.map_err(|e| -> CommonError { e.into() })?;
				if deposit_result {
					txn_status = TransactionStatus::SUCCESS;
				} else {
					txn_status = TransactionStatus::FAILED;
				}
			} else {
				txn_status = TransactionStatus::FAILED;
			}
		} else if let TransactionType::AccountOpen = transaction_type
		// Account_Open
		{
			// First create the user
			let data = transaction.transaction_data.clone();
			info!("data: {:?}",data);
			let mut user: CreateUser = from_value(data).unwrap();
			let created_user: User = self.user_repository.create(&mut user).await.map_err(|e| -> CommonError { e.into() })?;

			// Then create the account
			let account = CreateAccount {
				user_id: created_user.user_id,
				account_number: generate_account_number(created_user.user_id),
				account_type_id: 1,
				latest_transaction_hash: txn_hash.clone(),
			};
			self.account_repository.create(&account).await.map_err(|e| -> CommonError { e.into() })?;
			txn_status = TransactionStatus::SUCCESS;
		} else if let TransactionType::AccountUpdate = transaction_type
		// Account_Update
		{
			let data = transaction.transaction_data.clone();
			let user: UpdateUser = from_value(data).unwrap();
			let user_email = &user.email; // Immutable borrow for email
			let mut updated_user = user.clone(); // Mutable borrow for modification
			let user_update_result = self
				.user_repository
				.update(user_email, &mut updated_user)
				.await
				.map_err(|e| -> CommonError { e.into() });

			match user_update_result {
				Ok(_updated_user) => {
					txn_status = TransactionStatus::SUCCESS;
				}
				Err(_error) => {
					txn_status = TransactionStatus::FAILED;
				}
			};
		} else if let TransactionType::AccountClose = transaction_type
		// Account_Close
		{
			let data = transaction.transaction_data.clone();
			let account: DeleteAccount = from_value(data).unwrap();
			let user_account = &account.account_num; // Immutable borrow for email
			let user_account_close = self
				.account_repository
				.delete(user_account, account.clone())
				.await
				.map_err(|e| -> CommonError { e.into() });

			match user_account_close {
				Ok(_updated_user) => {
					txn_status = TransactionStatus::SUCCESS;
				}
				Err(_error) => {
					txn_status = TransactionStatus::FAILED;
				}
			};
		}
		let json_data = json!({
			"transaction_id": transaction.transaction_id,
			"user_id": transaction.user_id,
			"transaction_type": transaction_type.to_string(),
			"transaction_data": transaction.transaction_data,
			"transaction_hash": transaction.transaction_hash,
			"transaction_date": transaction.transaction_date,
			"block_number": block_number,
			"status": txn_status.to_string(),
		});
		let eve = CreateEvent {
			// Root user's id
			transaction_id: Some(transaction.transaction_id),
			event_type: transaction_type.to_string(),
			event_data: Some(json_data.clone()),
		};
		self.event_repository.create(&eve).await.map_err(|e| -> CommonError { e.into() })?;
		let updated_txn = UpdateTransaction { block_number, status: txn_status };
		self.txn_repository.update(&txn_hash, updated_txn).await.map_err(|e| -> CommonError { e.into() })
	}
}
