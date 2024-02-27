use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn generate_hash(value: &Value) -> String
{
	let mut hasher = Sha256::new();
	hasher.update(value.to_string().as_bytes());
	let hash = hasher.finalize();

	// Convert the hash to a string.
	let hash_string = hex::encode(hash);

	// Return the hash string.
	hash_string
}

pub fn generate_hash_from_vector(transaction_hashes: &Vec<String>) -> String
{
	let mut hasher = Sha256::new();
	for transaction_hash in transaction_hashes
	{
		hasher.update(transaction_hash.as_bytes());
	}
	let hash = hasher.finalize();

	// Convert the hash to a string.
	let hash_string = hex::encode(hash);

	// Return the hash string.
	hash_string
}
