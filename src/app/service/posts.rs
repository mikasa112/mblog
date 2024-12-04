use serde::Serialize;
use crate::app::model;
use crate::internal::result::ApiResult;
use crate::internal::result::response::{ListResponse, ObjResponse};

#[derive(Debug, Serialize)]
pub struct Posts {
    pub id: u32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn list(page: u32, size: u32) -> ApiResult<ListResponse<Posts>> {
    let list = model::posts::Posts::query_posts_list(size, (page - 1) * size).await?;
    let total = model::posts::Posts::query_posts_count().await?;
    let list = list.into_iter().map(|it| {
        Posts {
            id: it.id,
            content: it.content,
            created_at: format!("{}", it.created_at.format("%Y-%m-%d %H:%M:%S")),
            updated_at: format!("{}", it.updated_at.format("%Y-%m-%d %H:%M:%S")),
        }
    }).collect::<Vec<Posts>>();
    Ok(ListResponse {
        err_msg: None,
        status: 0,
        data: Some(list),
        total: Some(total as usize),
    })
}

pub async fn one_of_id(id: u32) -> ApiResult<ObjResponse<Posts>> {
    let it = model::posts::Posts::query_posts_by_id(id).await?;
    let it = Posts {
        id: it.id,
        content: it.content,
        created_at: format!("{}", it.created_at.format("%Y-%m-%d %H:%M:%S")),
        updated_at: format!("{}", it.updated_at.format("%Y-%m-%d %H:%M:%S")),
    };
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: Some(it),
    })
}