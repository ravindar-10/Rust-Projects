use std::sync::Arc;

use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};

use crate::{
	domain::{
		repositories::{
			account::AccountRepository, block::BlockRepository, event::EventRepository, transaction::TransactionRepository, user::UserRepository,
		},
		services::{
			account::AccountService, block::BlockService, event::EventService, orchestrator::OrchestratorService, transaction::TransactionService,
			user::UserService,
		},
	},
	infrastructure::{
		databases::pgsql::run_migration,
		repositories::{
			account::AccountDieselRepository, block::BlockDieselRepository, event::EventDieselRepository, transaction::TransactionDieselRepository,
			user::UserDieselRepository,
		},
	},
	services::{
		account::AccountServiceImpl, block::BlockServiceImpl, event::EventServiceImpl, orchestrator::OrchestratorServiceImpl,
		transaction::TransactionServiceImpl, user::UserServiceImpl,
	},
};

pub struct Container
{
	pub orchestrator_service: Arc<dyn OrchestratorService>,
	pub user_service: Arc<dyn UserService>,
	pub block_service: Arc<dyn BlockService>,
	pub account_service: Arc<dyn AccountService>,
	pub txn_service: Arc<dyn TransactionService>,
	pub event_service: Arc<dyn EventService>,
}

impl Container
{
	pub fn new(db_pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self
	{
		let mut connection = db_pool.get().expect("Could not fetch connection from pool");
		run_migration(&mut connection);
		let user_repository: Arc<dyn UserRepository> = Arc::new(UserDieselRepository::new(Arc::clone(&db_pool)));
		let account_repository: Arc<dyn AccountRepository> = Arc::new(AccountDieselRepository::new(Arc::clone(&db_pool)));
		let txn_repository: Arc<dyn TransactionRepository> = Arc::new(TransactionDieselRepository::new(Arc::clone(&db_pool)));
		let block_repository: Arc<dyn BlockRepository> = Arc::new(BlockDieselRepository::new(Arc::clone(&db_pool)));
		let event_repository: Arc<dyn EventRepository> = Arc::new(EventDieselRepository::new(Arc::clone(&db_pool)));
		let orchestrator_service: Arc<dyn OrchestratorService> = Arc::new(OrchestratorServiceImpl {
			user_repository: user_repository.clone(),
			account_repository: account_repository.clone(),
			txn_repository: txn_repository.clone(),
			block_repository: block_repository.clone(),
			event_repository: event_repository.clone(),
		});
		let user_service = Arc::new(UserServiceImpl { repository: user_repository.clone(), orchestrator_service: orchestrator_service.clone() });
		let block_service = Arc::new(BlockServiceImpl { repository: block_repository.clone(), orchestrator_service: orchestrator_service.clone() });
		let account_service = Arc::new(AccountServiceImpl { repository: account_repository.clone(), orchestrator_service: orchestrator_service.clone() });
		let txn_service = Arc::new(TransactionServiceImpl { repository: txn_repository.clone(), orchestrator_service: orchestrator_service.clone() });
		let event_service = Arc::new(EventServiceImpl { repository: event_repository.clone(), orchestrator_service: orchestrator_service.clone() });
		Container { orchestrator_service, user_service, block_service, account_service, txn_service, event_service }
	}
}
