use salvo::prelude::*;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use tracing::log::{log, Level};
use mblog::{init_config};
use mblog::app::database::db_pool;
use mblog::app::response::ListResponse;

#[derive(Debug, FromRow, Serialize)]
struct Post {
    content: String,
    // created_at: NaiveDateTime,
}

#[handler]
async fn hello(res: &mut Response) {
    let result: Vec<Post> = sqlx::query_as!(Post,
        r#"SELECT tp.content from t_posts tp;"#
    )
        .fetch_all(db_pool()).await.unwrap();
    res.render(Json(ListResponse {
        err_msg: None,
        status: 0,
        data: result,
        total: None,
    }));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    match init_config().await {
        Ok(config) => {
            let acceptor = TcpListener::new(format!("{}:{}", config.application.host, config.application.port)).bind().await;
            let server = Server::new(acceptor);
            // let handle = server.handle();
            // tokio::spawn(async move {
            //     tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            //     handle.stop_graceful(None);
            // });
            server.serve(Router::new().get(hello)).await;
        }
        Err(e) => {
            log!(Level::Error,"{}",e.to_string())
        }
    }
}
