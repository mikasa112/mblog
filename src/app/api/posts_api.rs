use crate::app::service::posts::{list, one_of_id, Posts};
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use crate::internal::result::code::Code;

#[handler]
pub async fn list_posts(req: &mut Request) -> ApiResult<ListResponse<Posts>> {
    let page = req.query::<u32>("page").unwrap_or(1);
    let size = req.query::<u32>("size").unwrap_or(10);
    if page <= 0 || size <= 0 {
        Err(Code::New(10001, "page或者size需要大于0".to_string()))
    } else {
        list(page, size).await
    }
}

#[handler]
pub async fn one_posts(req: &mut Request) -> ApiResult<ObjResponse<Posts>> {
    let id = req.params().get("id").unwrap_or(&String::from("1")).parse::<u32>().unwrap_or(1);
    one_of_id(id).await
}
