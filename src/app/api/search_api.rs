use crate::app::service;
use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use salvo::{handler, Request};

#[handler]
pub async fn search(req: &mut Request) -> ApiResult<ObjResponse<Vec<String>>> {
    if let Some(str) = req.query::<String>("query") {
        let page = req.query::<usize>("page").unwrap_or(1);
        let size = req.query::<usize>("size").unwrap_or(50);
        service::search::search_quick_content(page, size, str).await
    } else {
        Ok(ObjResponse {
            err_msg: None,
            status: 0,
            data: None,
        })
    }
}
