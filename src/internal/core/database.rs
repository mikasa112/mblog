use std::ops::Deref;
use sqlx::MySqlPool;
use std::sync::LazyLock;
use crate::internal::core::config::BLOG_CONFIG;

pub static DB_POOL: LazyLock<MySqlPool> = LazyLock::new(|| {
    MySqlPool::connect_lazy(BLOG_CONFIG.database.url.as_str()).unwrap()
});

pub fn db_pool() -> &'static MySqlPool {
    DB_POOL.deref()
}