use crate::app::service;
use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use salvo::{handler, Request};

#[handler]
pub async fn search(req: &mut Request) -> ApiResult<ObjResponse<Vec<String>>> {
    if let Some(str) = req.query::<String>("query") {
        let page = req.query::<u32>("page").unwrap_or(1);
        service::search::search_quick_content(page, str).await
    } else {
        Ok(ObjResponse {
            err_msg: None,
            status: 0,
            data: None,
        })
    }
}