use salvo::{handler, Request};
use crate::internal::result::ApiResult;
use crate::internal::result::response::ListResponse;
use crate::app::service;

#[handler]
pub async fn search(req: &mut Request) -> ApiResult<ListResponse<service::search::QuickContent>> {
    if let Some(str) = req.query::<String>("query") {
        let page = req.query::<u32>("page").unwrap_or(1);
        service::search::search_quick_content(page, str).await
    } else {
        Ok(ListResponse {
            err_msg: None,
            status: 0,
            data: None,
            total: None,
        })
    }
}