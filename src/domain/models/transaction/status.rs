use serde::{Deserialize, Serialize};

use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TransactionStatus
{
	PENDING,
	SUCCESS,
	FAILED,
}

impl fmt::Display for TransactionStatus
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{:?}", self)
	}
}

impl FromStr for TransactionStatus
{
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s
		{
			"PENDING" => Ok(TransactionStatus::PENDING),
			"SUCCESS" => Ok(TransactionStatus::SUCCESS),
			"FAILED" => Ok(TransactionStatus::FAILED),
			_ => Err(()),
		}
	}
}
