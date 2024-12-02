use std::error::Error;
use std::sync::OnceLock;
use sqlx::MySqlPool;

pub static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

pub fn db_pool() -> &'static MySqlPool {
    DB_POOL.get().unwrap()
}

pub async fn make_db_pool(db_url: &str) -> Result<MySqlPool, Box<dyn Error>> {
    Ok(MySqlPool::connect(db_url).await?)
}
