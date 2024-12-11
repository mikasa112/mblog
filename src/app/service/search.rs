use serde::{Deserialize, Serialize};
use crate::internal::result::ApiResult;
use crate::internal::result::response::ListResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickContent {
    pub post_id: u32,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
}

pub async fn search_quick_content(page: u32, query: String)
                                  -> ApiResult<ListResponse<QuickContent>> {


    Ok(ListResponse {
        err_msg: None,
        status: 0,
        data: None,
        total: None,
    })
}