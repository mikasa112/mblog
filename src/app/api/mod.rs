use salvo::Router;

pub mod account_api;
pub mod posts_api;
pub mod category_api;
pub mod tag_api;

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
        //单个文章
        .push(Router::with_path("posts/<id>").get(one_post))
        //分类列表
        .push(Router::with_path("categories").get(category_api::categories))

        .push(Router::with_path("tags").get(tag_api::tag_list))
}

fn auth_router() -> Router {
    Router::new()
        .hoop(auth_handler())
        //创建文章、更新文章
        .push(Router::with_path("posts").post(create_post).put(update_post))
        //创建分类
        .push(Router::with_path("category").post(category_api::create_category))
        //创建标签
        .push(Router::with_path("tags").post(tag_api::create_tag))
        //删除标签
        .push(Router::with_path("tags/<id>").delete(tag_api::delete_tag))
}

pub fn root_router() -> Router {
    Router::new()
        .hoop(LogMiddleware::new())
        .path("v1")
        .push(open_router())
        .push(auth_router())
}
