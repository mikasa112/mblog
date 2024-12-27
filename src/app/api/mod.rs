use salvo::cors::{Cors, CorsHandler};
use salvo::hyper::Method;
use salvo::prelude::StaticDir;
use salvo::Router;
use validator::ValidationError;

pub mod account_api;
pub mod category_api;
pub mod posts_api;
pub mod search_api;
pub mod tag_api;

use crate::app::api::posts_api::{
    create_post, create_post_tags, delete_post_tag, list_posts, one_post, update_post,
};
use crate::internal::core::config::BLOG_CONFIG;
use crate::internal::middleware::auth::auth_handler;
use crate::internal::middleware::catch_panic::CatchPanic;
use crate::internal::middleware::log::LogMiddleware;

fn open_router() -> Router {
    Router::new()
        //登录
        .push(Router::with_path("login").post(account_api::login))
        //用户信息
        .push(Router::with_path("userinfo").get(account_api::info))
        //文章列表
        .push(Router::with_path("posts").get(list_posts))
        //单个文章
        .push(Router::with_path("posts/<id>").get(one_post))
        //分类列表
        .push(Router::with_path("categories").get(category_api::categories))
        //标签列表
        .push(Router::with_path("tags").get(tag_api::tag_list))
        //单个标签
        .push(Router::with_path("tags/<id>").get(tag_api::get_tag))
        //搜索引擎
        .push(Router::with_path("search").get(search_api::search))
}

fn auth_router() -> Router {
    Router::new()
        .hoop(auth_handler())
        //创建文章、更新文章
        .push(
            Router::with_path("posts")
                .post(create_post)
                .put(update_post),
        )
        //绑定文章-标签、删除文章-标签
        .push(
            Router::with_path("posts/tags")
                .post(create_post_tags)
                .delete(delete_post_tag),
        )
        //创建分类
        .push(Router::with_path("category").post(category_api::create_category))
        //创建标签
        .push(Router::with_path("tags").post(tag_api::create_tag))
        //删除标签
        .push(Router::with_path("tags/<id>").delete(tag_api::delete_tag))
}

fn cors_handler() -> CorsHandler {
    Cors::new()
        .allow_origin(BLOG_CONFIG.application.allow_origin.as_str())
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .into_handler()
}

pub fn root_router() -> Router {
    Router::new()
        .push(
            Router::new()
                .hoop(CatchPanic::new())
                .hoop(LogMiddleware::new())
                .hoop(cors_handler())
                .path("v1")
                .push(open_router())
                .push(auth_router()),
        )
        .push(
            Router::with_path("<**path>")
                .get(StaticDir::new([BLOG_CONFIG.application.resource.as_str()]).auto_list(true)),
        )
}

pub fn id_validator(value: u32) -> Result<(), ValidationError> {
    if value >= 1 {
        Ok(())
    } else {
        Err(ValidationError::new("id需要大于0"))
    }
}
