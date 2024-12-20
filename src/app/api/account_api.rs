use crate::app::service::account::AccountParams;
use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use validator::Validate;

#[handler]
pub async fn login(req: &mut Request) -> ApiResult<ObjResponse<String>> {
    let params = req.parse_json::<AccountParams>().await?;
    params.validate()?;
    crate::app::service::account::login(params).await
}

// pub async fn account(req: &mut Request) -> ApiResult<ObjResponse<AccountParams>> {
//     Ok(ObjResponse {
//         err_msg: None,
//         status: 0,
//         data: None,
//     })
// }