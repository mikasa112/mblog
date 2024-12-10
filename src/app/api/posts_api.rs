use crate::app;
use crate::app::service::posts::{list, one_of_id, PostDetail, PostParams, PostTagsParams, Posts, UpdatePostParams};
use crate::internal::result::code::Code;
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use validator::Validate;

#[handler]
pub async fn list_posts(req: &mut Request) -> ApiResult<ListResponse<Posts>> {
    let page = req.query::<u32>("page").unwrap_or(1);
    let size = req.query::<u32>("size").unwrap_or(10);
    if page <= 0 || size <= 0 {
        Err(Code::New(10001, "page或者size需要大于0".to_string()))
    } else {
        list(page, size).await
    }
}

#[handler]
pub async fn one_post(req: &mut Request) -> ApiResult<ObjResponse<PostDetail>> {
    let id = req
        .params()
        .get("id")
        .unwrap_or(&String::from("1"))
        .parse::<u32>()
        .unwrap_or(1);
    one_of_id(id).await
}

#[handler]
pub async fn create_post(req: &mut Request) -> ApiResult<ObjResponse<()>> {
    let params = req.parse_json::<PostParams>().await?;
    params.validate()?;
    app::service::posts::create_post(params).await
}

#[handler]
pub async fn update_post(req: &mut Request) -> ApiResult<ObjResponse<()>> {
    let params = req.parse_json::<UpdatePostParams>().await?;
    params.validate()?;
    app::service::posts::update_post(params).await
}

#[handler]
pub async fn create_post_tags(req: &mut Request) -> ApiResult<ObjResponse<()>> {
    let params = req.parse_json::<PostTagsParams>().await?;
    params.validate()?;
    app::service::posts::create_post_tag(params).await
}

#[handler]
pub async fn delete_post_tag(req: &mut Request) -> ApiResult<ObjResponse<()>> {
    let params = req.parse_json::<PostTagsParams>().await?;
    params.validate()?;
    app::service::posts::delete_post_tag(params).await
}