use salvo::prelude::*;
use tracing::log::{log, Level};
use mblog::{init_config};

#[handler]
async fn hello() -> &'static str {
    "hello world"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    match init_config().await {
        Ok(config) => {
            let router = Router::new().get(hello);
            let acceptor = TcpListener::new(format!("{}:{}", config.application.host, config.application.port)).bind().await;
            Server::new(acceptor).serve(router).await;
        }
        Err(e) => {
            log!(Level::Error,"{}",e.to_string())
        }
    }
}
