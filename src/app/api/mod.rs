use salvo::Router;

pub mod posts_api;
use posts_api::{list_posts, one_posts};
use crate::app::api::posts_api::create_posts;
use crate::internal::middleware::auth::{auth_handler};
use crate::internal::middleware::log::LogMiddleware;

fn open_router() -> Router {
    Router::new()
        //文章列表
        .push(Router::with_path("posts").get(list_posts))
        .push(Router::with_path("posts/<id>").get(one_posts))
        .push(Router::with_path("posts").post(create_posts))
}

fn auth_router() -> Router {
    Router::new()
        .hoop(auth_handler())
}

pub fn root_router() -> Router {
    Router::new()
        .hoop(LogMiddleware::new())
        .path("v1")
        .push(open_router())
        .push(auth_router())
}

