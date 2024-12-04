use salvo::Router;

pub mod posts_api;
use posts_api::{list_posts, one_posts};
use crate::internal::middleware::log::LogMiddleware;

pub fn root_router() -> Router {
    Router::new()
        .hoop(LogMiddleware::new())
        //文章列表
        .push(Router::with_path("posts").get(list_posts))
        .push(Router::with_path("posts/<id>").get(one_posts))
}