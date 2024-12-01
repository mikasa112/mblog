use mblog::database::{db_pool, make_db_pool, DB_POOL};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::log::{log, Level};

#[handler]
async fn hello() -> &'static str {
    "hello world"
}

#[derive(Debug, Serialize, Deserialize)]
struct MyBlogConfig {
    database: Database,
}
#[derive(Debug, Serialize, Deserialize)]
struct Database {
    url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    let config: MyBlogConfig =
        serde_yml::from_str(include_str!("/home/mikasa/code/mblog/mblog.yaml"))
            .expect("failed to load config.yml");
    let pool = make_db_pool(config.database.url.as_str()).await;
    if let Ok(pool_impl) = pool {
        DB_POOL.set(pool_impl).unwrap();
    }
    log!(Level::Info,"{:?}",db_pool().options());
    Server::new(acceptor).serve(router).await;
}
