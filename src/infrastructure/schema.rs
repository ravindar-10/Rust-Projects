// @generated automatically by Diesel CLI.

diesel::table! {
		account_types (account_type_id) {
				account_type_id -> Int4,
				#[max_length = 50]
				type_name -> Varchar,
		}
}

diesel::table! {
		accounts (account_id) {
				account_id -> Int4,
				#[max_length = 20]
				account_number -> Varchar,
				user_id -> Nullable<Int4>,
				balance -> Float8,
				account_type_id -> Nullable<Int4>,
				latest_transaction_hash -> Nullable<Varchar>,
				created_date -> Timestamp,
				updated_date -> Timestamp,
				is_deleted -> Bool,
		}
}

diesel::table! {
		blocks (block_number) {
				block_number -> Int4,
				#[max_length = 64]
				block_hash -> Varchar,
				transaction_count -> Int4,
				created_at -> Timestamp,
		}
}

diesel::table! {
		events (event_id) {
				event_id -> Int4,
				transaction_id -> Nullable<Int4>,
				#[max_length = 20]
				event_type -> Varchar,
				event_data -> Nullable<Json>,
				created_at -> Nullable<Timestamp>,
		}
}

diesel::table! {
		transaction_types (transaction_type_id) {
				transaction_type_id -> Int4,
				#[max_length = 50]
				type_name -> Varchar,
		}
}

diesel::table! {
		transactions (transaction_id) {
				transaction_id -> Int4,
				user_id -> Nullable<Int4>,
				transaction_type_id -> Nullable<Int4>,
				transaction_data -> Jsonb,
				#[max_length = 64]
				transaction_hash -> Varchar,
				transaction_date -> Timestamp,
				block_number -> Nullable<Int4>,
				#[max_length = 42]
				status -> Varchar,
		}
}

diesel::table! {
		users (user_id) {
				user_id -> Int4,
				#[max_length = 255]
				email -> Varchar,
				#[max_length = 100]
				first_name -> Varchar,
				#[max_length = 100]
				last_name -> Varchar,
				nonce -> Int4,
				registration_date -> Timestamp,
		}
}

diesel::allow_tables_to_appear_in_same_query!(account_types, accounts, blocks, events, transaction_types, transactions, users,);
