use mblog::app::api::root_router;
use mblog::internal::core::config::BLOG_CONFIG;
use salvo::prelude::*;
// use mblog::internal::core::job::MyJob;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    // MyJob::new().await.unwrap();
    let acceptor = TcpListener::new(format!(
        "{}:{}",
        BLOG_CONFIG.application.host, BLOG_CONFIG.application.port
    ))
        .bind()
        .await;
    let server = Server::new(acceptor);
    server.serve(root_router()).await;
}
