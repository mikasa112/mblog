use crate::app::api::category_api::CategoryParams;
use crate::app::model;
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use serde::Serialize;

// #[derive(Serialize, Debug)]
// pub struct Category {
//     pub id: u32,
//     pub name: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub description: Option<String>,
//     pub created_at: String,
//     pub updated_at: String,
// }

#[derive(Serialize, Debug)]
pub struct CategorySimpler {
    pub id: u32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub post_count: i64,
}

pub async fn list() -> ApiResult<ListResponse<CategorySimpler>> {
    let count = model::category::CategorySimpler::count().await?;
    let list = model::category::CategorySimpler::list()
        .await?
        .into_iter()
        .map(|it| CategorySimpler {
            id: it.id,
            name: it.name,
            description: it.description,
            post_count: it.post_count,
        })
        .collect();
    Ok(ListResponse {
        err_msg: None,
        status: 0,
        data: Some(list),
        total: Some(count),
    })
}

pub async fn create(params: CategoryParams) -> ApiResult<ObjResponse<()>> {
    model::category::Category::create(params.name, params.description).await?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: None,
    })
}
