use mblog::app::api::my_service;
use mblog::internal::core::config::BLOG_CONFIG;
use mblog::internal::core::jobs::AsyncDatabaseJob;
use mblog::internal::utils::log_utils::Logger;
use salvo::prelude::*;

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
    Server::new(acceptor).serve(my_service()).await;
}
