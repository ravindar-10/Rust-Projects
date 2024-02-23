use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithdrawalTransaction {
	pub from_account_number: String,
	pub amount: f64,
}

impl Into<WithdrawalTransaction> for Value {
	fn into(self) -> WithdrawalTransaction {
		WithdrawalTransaction {
			from_account_number: self["from_account_number"].as_str().unwrap().to_string(),
			amount: self["amount"].as_f64().unwrap(),
		}
	}
}
