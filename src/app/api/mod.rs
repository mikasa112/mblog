use salvo::Router;

pub mod account_api;
pub mod posts_api;

use crate::app::api::posts_api::{create_post, update_post};
use crate::internal::middleware::auth::auth_handler;
use crate::internal::middleware::log::LogMiddleware;
use posts_api::{list_posts, one_post};

fn open_router() -> Router {
    Router::new()
        //登录
        .push(Router::with_path("login").post(account_api::login))
        //文章列表
        .push(Router::with_path("posts").get(list_posts))
        .push(Router::with_path("post/<id>").get(one_post))
}

fn auth_router() -> Router {
    Router::new()
        .hoop(auth_handler())
        .push(Router::with_path("post").post(create_post))
        .push(Router::with_path("post").put(update_post))
}

pub fn root_router() -> Router {
    Router::new()
        .hoop(LogMiddleware::new())
        .path("v1")
        .push(open_router())
        .push(auth_router())
}
