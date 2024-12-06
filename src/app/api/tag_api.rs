use crate::app::service;
use crate::app::service::tag::{Tag, TagAndPost};
use crate::internal::result::code::Code;
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use validator::Validate;

#[handler]
pub async fn create_tag(req: &mut Request) -> ApiResult<ObjResponse<()>> {
    let params = req.parse_json::<service::tag::TagParams>().await?;
    params.validate()?;
    service::tag::create_tag(params).await
}

#[handler]
pub async fn delete_tag(req: &mut Request) -> ApiResult<ObjResponse<()>> {
    if let Some(id) = req.params().get("id") {
        if let Ok(tag_id) = id.parse::<u32>() {
            return service::tag::delete_tag(tag_id).await;
        };
    };
    Err(Code::SimpleParamsError)
}

#[handler]
pub async fn tag_list() -> ApiResult<ListResponse<Tag>> {
    service::tag::list().await
}

#[handler]
pub async fn get_tag(req: &mut Request) -> ApiResult<ObjResponse<TagAndPost>> {
    if let Some(id) = req.params().get("id") {
        if let Ok(tag_id) = id.parse::<u32>() {
            return service::tag::tag(tag_id).await;
        };
    };
    Err(Code::SimpleParamsError)
}
