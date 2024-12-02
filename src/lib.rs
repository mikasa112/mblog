use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::database::{make_db_pool, DB_POOL};

pub mod database;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyBlogConfig {
    pub application: Application,
    pub database: Database,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub host: String,
    pub port: usize,
}

pub async fn init_config() -> Result<MyBlogConfig, Box<dyn Error>> {
    let config: MyBlogConfig = serde_yml::from_str(include_str!("D:/code/mblog/mblog.yaml"))?;
    let pool = make_db_pool(config.database.url.as_str()).await?;
    DB_POOL.set(pool).unwrap();
    Ok(config)
}