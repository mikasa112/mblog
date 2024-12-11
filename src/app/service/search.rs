use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use serde::{Deserialize, Serialize};
use crate::internal::core::tantivy_engine::SEARCH_ENGINE;
use crate::internal::result::code::Code;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickContent {
    pub post_id: u32,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
}

pub async fn search_quick_content(page: u32, query: String)
                                  -> ApiResult<ObjResponse<Vec<String>>> {
    match &*SEARCH_ENGINE {
        Ok(engine) => {
            engine.search(query.as_str(), 20, page as usize)?;
            Ok(ObjResponse {
                err_msg: None,
                status: 0,
                data: None,
            })
        }
        Err(err) => {
            Err(Code::New(10004, format!("初始化搜索引擎错误：{}", err.to_string())))
        }
    }
}