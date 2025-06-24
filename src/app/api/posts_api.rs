use crate::app;
use crate::app::service::posts::{
    list, one_of_id, DeletePostTagParams, PostDetail, PostParams, PostTagsParams, Posts,
    SearchStatus, UpdatePostParams,
};
use crate::internal::result::code::Code;
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use validator::Validate;

#[handler]
pub async fn list_posts_pub(req: &mut Request) -> ApiResult<ListResponse<Posts>> {
    let page = req.query::<u32>("page").unwrap_or(1);
    let size = req.query::<u32>("size").unwrap_or(10);
    list(SearchStatus::Published, page, size).await
}

#[handler]
pub async fn list_posts_auth(req: &mut Request) -> ApiResult<ListResponse<Posts>> {
    let page = req.query::<u32>("page").unwrap_or(1);
    let size = req.query::<u32>("size").unwrap_or(10);
    list(SearchStatus::All, page, size).await
}

#[handler]
pub async fn one_post(req: &mut Request) -> ApiResult<ObjResponse<PostDetail>> {
    if let Some(id_str) = req.params().get("id") {
        if let Ok(id) = id_str.parse::<u32>() {
            one_of_id(id).await
        } else {
            Err(Code::SimpleParamsError)
        }
    } else {
        Err(Code::SimpleParamsError)
    }
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
    let params = req.parse_json::<DeletePostTagParams>().await?;
    params.validate()?;
    app::service::posts::delete_post_tag(params).await
}
