use serde::{Deserialize, Serialize};

use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TransactionType {
	Deposit = 1,
	Withdrawal = 2,
	Transfer = 3,
	AccountOpen = 4,
	AccountUpdate = 5,
	AccountClose = 6,
	Invalid = 7,
}

impl fmt::Display for TransactionType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl FromStr for TransactionType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"AccountOpen" => Ok(TransactionType::AccountOpen),
			"AccountClose" => Ok(TransactionType::AccountClose),
			"AccountUpdate" => Ok(TransactionType::AccountUpdate),
			"Deposit" => Ok(TransactionType::Deposit),
			"Withdrawal" => Ok(TransactionType::Withdrawal),
			"Transfer" => Ok(TransactionType::Transfer),
			_ => Ok(TransactionType::Invalid),
		}
	}
}

impl From<i32> for TransactionType {
	fn from(value: i32) -> Self {
		match value {
			1 => TransactionType::Deposit,
			2 => TransactionType::Withdrawal,
			3 => TransactionType::Transfer,
			4 => TransactionType::AccountOpen,
			5 => TransactionType::AccountUpdate,
			6 => TransactionType::AccountClose,
			_ => TransactionType::Invalid,
		}
	}
}
