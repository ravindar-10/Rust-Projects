use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferTransaction {
	pub from_account_number: String,
	pub to_account_number: String,
	pub amount: f64,
}

impl Into<TransferTransaction> for Value {
	fn into(self) -> TransferTransaction {
		TransferTransaction {
			from_account_number: self["from_account_number"].as_str().unwrap().to_string(),
			to_account_number: self["to_account_number"].as_str().unwrap().to_string(),
			amount: self["amount"].as_f64().unwrap(),
		}
	}
}
