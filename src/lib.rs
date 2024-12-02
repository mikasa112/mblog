use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::LazyLock;

pub mod app;
pub mod api;
pub mod model;
pub static BLOG_CONFIG: LazyLock<MyBlogConfig> = LazyLock::new(|| {
    let config: MyBlogConfig = serde_yml::from_str(include_str!("D:/code/mblog/mblog.yaml")).unwrap();
    config
});

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