use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::internal::result::ApiResult;
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::app::model;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TagParams {
    #[validate(length(min = 1, message = "name不能为空"))]
    pub name: String,
}
#[derive(Debug, Serialize)]
pub struct Tag {
    pub id: u32,
    pub name: String,
}

pub async fn create_tag(params: TagParams) -> ApiResult<ObjResponse<()>> {
    model::tag::Tag::create(params.name).await?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: None,
    })
}

pub async fn delete_tag(id: u32) -> ApiResult<ObjResponse<()>> {
    model::tag::Tag::delete(id).await?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: None,
    })
}

pub async fn list() -> ApiResult<ListResponse<Tag>> {
    let list = model::tag::Tag::list().await?.into_iter().map(|it| {
        Tag {
            id: it.id,
            name: it.name,
        }
    }).collect();
    Ok(ListResponse {
        err_msg: None,
        status: 0,
        data: Some(list),
        total: None,
    })
}

#[derive(Debug, Serialize)]
pub struct TagAndPost {
    pub id: u32,
    pub name: String,
    pub post_id: u32,
    pub post_title: String,
}

pub async fn tag(id: u32) -> ApiResult<ObjResponse<TagAndPost>> {

}