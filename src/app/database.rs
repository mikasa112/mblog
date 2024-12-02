use crate::BLOG_CONFIG;
use sqlx::MySqlPool;
use std::sync::OnceLock;

pub static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

pub fn db_pool() -> &'static MySqlPool {
    DB_POOL.get().unwrap()
}

pub async fn init_db() {
    DB_POOL.set(MySqlPool::connect(BLOG_CONFIG.database.url.as_str()).await.unwrap()).unwrap()
}