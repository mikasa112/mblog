use mblog::app::api::root_router;
use mblog::internal::core::config::BLOG_CONFIG;
use mblog::internal::core::jobs::AsyncDatabaseJob;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    AsyncDatabaseJob::async_search_engine();
    let acceptor = TcpListener::new(format!(
        "{}:{}",
        BLOG_CONFIG.application.host, BLOG_CONFIG.application.port
    ))
    .bind()
    .await;
    let server = Server::new(acceptor);
    server.serve(root_router()).await;
}
