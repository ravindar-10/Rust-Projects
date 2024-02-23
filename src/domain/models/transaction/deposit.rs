use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DepositTransaction {
	pub to_account_number: String,
	pub amount: f64,
}

impl Into<DepositTransaction> for Value {
	fn into(self) -> DepositTransaction {
		DepositTransaction {
			to_account_number: self["to_account_number"].as_str().unwrap().to_string(),
			amount: self["amount"].as_f64().unwrap(),
		}
	}
}
