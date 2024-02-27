use std::{format, string::String};

pub fn generate_account_number(user_id: i32) -> String
{
	let prefix = "IRONCLAD";
	let user_id_str = format!("{:010}", user_id);
	let total_length = 20;

	if prefix.len() + user_id_str.len() > total_length
	{
		// If the combined length of prefix and user_id_str exceeds 20 characters,
		// truncate the prefix to make room for the user_id_str.
		let truncated_prefix = &prefix[..total_length - user_id_str.len()];
		format!("{}{}", truncated_prefix, user_id_str)
	}
	else
	{
		format!("{}{}", prefix, user_id_str)
	}
}
