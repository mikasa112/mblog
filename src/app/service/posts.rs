use crate::app::model;
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use crate::internal::utils::date_utils;
use serde::{Deserialize, Serialize};

use crate::app::model::posts::{MStatus, Status};
use crate::internal::core::tantivy_engine::PostDocument;
use crate::internal::result::code::Code;
use crate::internal::utils::id_validator;
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
            created_at: date_utils::naive_date_format(it.created_at),
            updated_at: date_utils::naive_date_format(it.updated_at),
        }
    }
}

pub enum SearchStatus {
    All,
    Published,
}

pub async fn list(status: SearchStatus, page: u32, size: u32) -> ApiResult<ListResponse<Posts>> {
    let (total, list) = match status {
        SearchStatus::All => {
            let list = model::posts::PostCategory::query_all_posts(size, (page - 1) * size).await?;
            let total = model::posts::Post::query_posts_count().await?;
            let list = list.into_iter().map(|it| it.into()).collect::<Vec<Posts>>();
            (total, list)
        }
        SearchStatus::Published => {
            let list = model::posts::PostCategory::query_posts_list(
                Status::Published,
                size,
                (page - 1) * size,
            )
            .await?;
            let total = model::posts::Post::query_publish_posts_count().await?;
            let list = list.into_iter().map(|it| it.into()).collect::<Vec<Posts>>();
            (total, list)
        }
    };
    Ok(ListResponse {
        err_msg: None,
        status: 0,
        data: Some(list),
        total: Some(total as usize),
    })
}

#[derive(Serialize, Debug)]
pub struct PostDetail {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn one_of_id(id: u32) -> ApiResult<ObjResponse<PostDetail>> {
    if let Some(it) = model::posts::PDetail::query_posts_by_id(id).await? {
        return Ok(ObjResponse {
            err_msg: None,
            status: 0,
            data: Some(PostDetail {
                id: it.id,
                category_name: it.category_name,
                title: it.title,
                content: it.content,
                excerpt: it.excerpt,
                tags: it.tags,
                created_at: date_utils::naive_date_format(it.created_at),
                updated_at: date_utils::naive_date_format(it.updated_at),
            }),
        });
    };
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: None,
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
    pub tag_id: Vec<u32>,
}

pub async fn create_post(params: PostParams) -> ApiResult<ObjResponse<()>> {
    //插入文章
    let id = model::posts::Post::insert_post(
        params.category_id,
        &params.title,
        &params.content,
        &params.excerpt,
    )
    .await?;
    //绑定标签
    model::posts::Post::insert_post_tag(&(id as u32), &params.tag_id).await?;
    if let Some(engine) = crate::internal::core::tantivy_engine::SEARCH_ENGINE.get() {
        engine.insert_batch(vec![PostDocument {
            id,
            title: params.title,
            content: params.content,
            excerpt: params.excerpt.unwrap_or_default(),
        }])?;
        Ok(ObjResponse {
            err_msg: None,
            status: 0,
            data: None,
        })
    } else {
        Err(Code::New(99996, "搜索引擎内部错误".to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePostParams {
    #[validate(custom(function = "id_validator"))]
    pub id: u32,
    pub title: Option<String>,
    pub category_id: Option<u32>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub status: MStatus,
}

pub async fn update_post(params: UpdatePostParams) -> ApiResult<ObjResponse<()>> {
    model::posts::Post::update_post(
        params.id,
        params.category_id,
        &params.title,
        &params.content,
        &params.excerpt,
        params.status,
    )
    .await?;
    if let Some(engine) = crate::internal::core::tantivy_engine::SEARCH_ENGINE.get() {
        tokio::spawn(async move {
            let p = model::posts::Post::query_posts_by_id(params.id).await?;
            engine.update(p.id as u64, p.title, p.content, p.excerpt)?;
            Ok(ObjResponse {
                err_msg: None,
                status: 0,
                data: None,
            })
        })
        .await
        .unwrap()
    } else {
        Err(Code::New(99996, "搜索引擎内部错误".to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostTagsParams {
    #[validate(custom(function = "id_validator"))]
    pub post_id: u32,
    pub tag_id: Vec<u32>,
}

/// 对文章绑定标签
pub async fn create_post_tag(params: PostTagsParams) -> ApiResult<ObjResponse<()>> {
    model::posts::Post::insert_post_tag(&params.post_id, &params.tag_id).await?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: None,
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeletePostTagParams {
    #[validate(custom(function = "id_validator"))]
    pub post_id: u32,
    #[validate(custom(function = "id_validator"))]
    pub tag_id: u32,
}

/// 删除{post_id}文章的{tag_id}标签
pub async fn delete_post_tag(params: DeletePostTagParams) -> ApiResult<ObjResponse<()>> {
    model::posts::Post::delete_post_tag(params.post_id, params.tag_id).await?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: None,
    })
}
