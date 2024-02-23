use utoipa::OpenApi;

use crate::api::{
	controllers::{
		account_handler::__path_read_account,
		block_handler::{__path_create_block, __path_list_blocks, __path_read_block},
		transaction_handler::{__path_create_transaction, __path_list_transactions, __path_read_transaction},
		user_handler::{__path_create_user, __path_delete_user, __path_list_users, __path_read_user, __path_update_user},
	},
	dto::{
		account::AccountDTO,
		block::BlockDTO,
		transaction::{CreateTransactionDTO, TransactionDTO},
		user::{CreateUserDTO, UpdateUserDTO, UserDTO},
	},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user, read_user, update_user, delete_user, list_users, create_block, read_block, list_blocks, read_account, create_transaction, read_transaction, list_transactions,
    ),
    components(
        schemas(CreateUserDTO, UserDTO, UpdateUserDTO, BlockDTO, AccountDTO, TransactionDTO, CreateTransactionDTO)
    ),
    tags(
        (name = "Rusty-Chain", description = "Rusty Chain management endpoints.")
    )
)]
pub struct ApiDoc;
