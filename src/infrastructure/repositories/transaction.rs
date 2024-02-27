use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
	domain::{
		models::transaction::{CreateTransaction, Transaction, UpdateTransaction},
		repositories::{
			repository::{QueryParams, RepositoryResult, ResultPaging},
			transaction::{TransactionQueryParams, TransactionRepository},
		},
	},
	infrastructure::{
		databases::pgsql::DBConn,
		error::DieselRepositoryError,
		models::transaction::{CreateTransactionDiesel, TransactionDiesel, UpdateTransactionDiesel},
	},
};
use std::sync::Arc;

pub struct TransactionDieselRepository
{
	pub pool: Arc<DBConn>,
}

impl TransactionDieselRepository
{
	pub fn new(db: Arc<DBConn>) -> Self
	{
		TransactionDieselRepository { pool: db }
	}
}

#[async_trait]
impl TransactionRepository for TransactionDieselRepository
{
	async fn create(&self, txn: &CreateTransaction) -> RepositoryResult<Transaction>
	{
		use crate::infrastructure::schema::transactions::dsl::transactions;
		let mut conn = self.pool.get().unwrap();
		let new_transaction_diesel: CreateTransactionDiesel = CreateTransactionDiesel::from(txn.clone());
		let result = diesel::insert_into(transactions)
			.values(new_transaction_diesel)
			.get_result::<TransactionDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(result.into())
	}

	async fn read(&self, txn_hash: &str) -> RepositoryResult<Transaction>
	{
		use crate::infrastructure::schema::transactions::dsl::{transaction_hash, transactions};
		let mut conn = self.pool.get().unwrap();
		transactions
			.filter(transaction_hash.eq(txn_hash))
			.first::<TransactionDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())
			.map(|v| -> Transaction { v.into() })
	}

	async fn update(&self, txn_hash: &str, txn: UpdateTransaction) -> RepositoryResult<Transaction>
	{
		use crate::infrastructure::schema::transactions::dsl::{transaction_hash, transactions};
		let update_transaction_diesel = UpdateTransactionDiesel::from(txn);
		let mut conn = self.pool.get().unwrap();
		let updated_transaction = diesel::update(transactions.filter(transaction_hash.eq(txn_hash)))
			.set(&update_transaction_diesel)
			.get_result::<TransactionDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(updated_transaction.into())
	}

	async fn list(&self, params: TransactionQueryParams) -> RepositoryResult<ResultPaging<Transaction>>
	{
		use crate::infrastructure::schema::transactions::dsl::{status, transactions};
		let mut conn = self.pool.get().unwrap();
		let builder = transactions
			.limit(params.limit())
			.offset(params.offset())
			.filter(status.eq(params.status.unwrap_or("Pending".to_string())));
		let result = builder
			.load::<TransactionDiesel>(&mut conn)
			.map_err(|v| DieselRepositoryError::from(v).into_inner())?;
		Ok(ResultPaging { total: result.len() as i64, items: result.into_iter().map(|v| v.into()).collect() })
	}
}
