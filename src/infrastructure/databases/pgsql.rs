use diesel::{self, pg::PgConnection, r2d2, r2d2::ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

use std::env;

use crate::domain::constants::POSTGRESQL_DB_POOL_SIZE_PER_WORKER;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn db_pool(database_url: &str) -> DBConn
{
	dotenv().ok();
	let pool_size_per_worker: u32 = env::var(POSTGRESQL_DB_POOL_SIZE_PER_WORKER)
		.expect(&*format!("{value} must be set", value = POSTGRESQL_DB_POOL_SIZE_PER_WORKER))
		.parse()
		.expect("Failed to parse pool size per worker as u32");
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	Pool::builder().max_size(pool_size_per_worker).build(manager).expect("Should create pool")
}

pub fn run_migration(connection: &mut PgConnection)
{
	connection.run_pending_migrations(MIGRATIONS).unwrap();
}
