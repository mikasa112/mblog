use crate::app::service::account::{AccountParams, UserInfo};
use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use validator::Validate;
use crate::app::service::account;

#[handler]
pub async fn login(req: &mut Request) -> ApiResult<ObjResponse<String>> {
    let params = req.parse_json::<AccountParams>().await?;
    params.validate()?;
    account::login(params).await
}

#[handler]
pub async fn info(_req: &mut Request) -> ApiResult<ObjResponse<UserInfo>> {
    account::user_info().await
}