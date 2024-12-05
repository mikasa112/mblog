use serde::{Deserialize, Serialize};
use crate::app::model;

use crate::internal::result::ApiResult;
use crate::internal::result::response::{ListResponse, ObjResponse};

use validator::Validate;

#[derive(Debug, Serialize)]
pub struct Posts {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<model::posts::PostCategory> for Posts {
    fn from(it: model::posts::PostCategory) -> Self {
        Posts {
            id: it.id,
            category_name: it.category_name,
            title: it.title,
            content: it.content,
            excerpt: it.excerpt,
            created_at: format!("{}", it.created_at.format("%Y-%m-%d %H:%M:%S")),
            updated_at: format!("{}", it.updated_at.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}

pub async fn list(page: u32, size: u32) -> ApiResult<ListResponse<Posts>> {
    let list = model::posts::PostCategory::query_posts_list(size, (page - 1) * size).await?;
    let total = model::posts::Post::query_posts_count().await?;
    let list = list.into_iter().map(|it| {
        it.into()
    }).collect::<Vec<Posts>>();
    Ok(ListResponse {
        err_msg: None,
        status: 0,
        data: Some(list),
        total: Some(total as usize),
    })
}

pub async fn one_of_id(id: u32) -> ApiResult<ObjResponse<Posts>> {
    let it = model::posts::PostCategory::query_posts_by_id(id).await?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: Some(it.into()),
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostParams {
    #[validate(length(min = 1, message = "title不能为空"))]
    pub title: String,
    pub category_id: Option<u32>,
    #[validate(length(min = 1, message = "content不能为空"))]
    pub content: String,
    pub excerpt: Option<String>,
}

pub async fn create_post(params: PostParams) -> ApiResult<ObjResponse<()>> {
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: None,
    })
}