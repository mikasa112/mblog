use crate::internal::core::tantivy_engine::SEARCH_ENGINE;
use crate::internal::result::code::Code;
use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickContent {
    pub post_id: u32,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
}

pub async fn search_quick_content(
    page: usize,
    size: usize,
    query: String,
) -> ApiResult<ObjResponse<Vec<String>>> {
    if let Some(engine) = SEARCH_ENGINE.get() {
        tokio::task::spawn_blocking(move || {
            let vec = engine.search(query.as_str(), size, (page - 1) * size)?;
            return Ok(ObjResponse {
                err_msg: None,
                status: 0,
                data: Some(vec),
            });
        })
        .await
        .unwrap()
    } else {
        Err(Code::New(10004, "初始化搜索引擎错误".to_string()))
    }
}
