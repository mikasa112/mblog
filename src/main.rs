use mblog::app::api::root_router;
use mblog::internal::core::config::BLOG_CONFIG;
use mblog::internal::core::jobs::AsyncDatabaseJob;
use salvo::prelude::*;
use mblog::internal::utils::log_utils::Logger;

#[tokio::main]
async fn main() {
    let _logger = Logger::init();
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
