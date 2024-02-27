use crate::domain::{
	error::ValidationError,
	models::transaction::{
		deposit::DepositTransaction, transfer::TransferTransaction, txn_type::TransactionType, withdrawal::WithdrawalTransaction, CreateTransaction,
	},
};

pub trait ValidateTransaction
{
	fn validate(&self) -> Result<(), ValidationError>;
}

impl ValidateTransaction for CreateTransaction
{
	fn validate(&self) -> Result<(), ValidationError>
	{
		if self.transaction_data.is_null()
		{
			return Err(ValidationError { message: "Transaction data is empty".to_string() });
		}
		if self.user_id < 1
		{
			return Err(ValidationError { message: "Invalid user id".to_string() });
		}
		if TransactionType::Deposit == self.transaction_type
		{
			let transaction: DepositTransaction =
				serde_json::from_value(self.transaction_data.clone()).map_err(|e| ValidationError { message: e.to_string() })?;
			if transaction.to_account_number.is_empty()
			{
				return Err(ValidationError { message: "To account number is empty".to_string() });
			}
			if transaction.amount < 0.1
			{
				return Err(ValidationError { message: "Invalid amount".to_string() });
			}
		}
		if TransactionType::Withdrawal == self.transaction_type
		{
			let transaction: WithdrawalTransaction =
				serde_json::from_value(self.transaction_data.clone()).map_err(|e| ValidationError { message: e.to_string() })?;
			if transaction.from_account_number.is_empty()
			{
				return Err(ValidationError { message: "From account number is empty".to_string() });
			}
			if transaction.amount < 0.1
			{
				return Err(ValidationError { message: "Invalid amount".to_string() });
			}
		}
		if TransactionType::Transfer == self.transaction_type
		{
			let transaction: TransferTransaction =
				serde_json::from_value(self.transaction_data.clone()).map_err(|e| ValidationError { message: e.to_string() })?;
			if transaction.from_account_number.is_empty()
			{
				return Err(ValidationError { message: "From account number is empty".to_string() });
			}
			if transaction.to_account_number.is_empty()
			{
				return Err(ValidationError { message: "To account number is empty".to_string() });
			}
			if transaction.amount < 0.1
			{
				return Err(ValidationError { message: "Invalid amount".to_string() });
			}
		}
		Ok(())
	}
}
