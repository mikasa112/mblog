use sqlx::SqlitePool;
use std::error::Error;
use std::sync::OnceLock;

pub static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub fn db_pool() -> &'static SqlitePool {
    DB_POOL.get().unwrap()
}

pub async fn make_db_pool(db_url: &str) -> Result<SqlitePool, Box<dyn Error>> {
    Ok(SqlitePool::connect(db_url).await?)
}
