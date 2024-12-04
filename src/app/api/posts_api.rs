use crate::app::service::posts::{list, one_of_id, Posts};
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use salvo::{handler, Request};

#[handler]
pub async fn list_posts(req: &mut Request) -> ApiResult<ListResponse<Posts>> {
    let page = req.query::<u32>("page").unwrap_or(1);
    let size = req.query::<u32>("size").unwrap_or(10);
    list(page, size).await
}

#[handler]
pub async fn one_posts(req: &mut Request) -> ApiResult<ObjResponse<Posts>> {
    let id = req.params().get("id").unwrap().parse::<u32>().unwrap();
    one_of_id(id).await
}