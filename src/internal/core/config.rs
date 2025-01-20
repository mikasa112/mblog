use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

pub static BLOG_CONFIG: LazyLock<MyBlogConfig> =
    LazyLock::new(|| serde_yml::from_str(include_str!("D:/code/mblog/mblog.yaml")).unwrap());

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
    pub secret_key: String,
    pub search_engine_dir: String,
    pub allow_origin: String,
    pub concurrency_limit: usize,
    pub resource: String,
}
