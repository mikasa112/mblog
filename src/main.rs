use salvo::prelude::*;
use mblog::api::root_router;
use mblog::BLOG_CONFIG;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let acceptor = TcpListener::new(format!("{}:{}", BLOG_CONFIG.application.host, BLOG_CONFIG.application.port)).bind().await;
    let server = Server::new(acceptor);
    server.serve(root_router()).await;
}
