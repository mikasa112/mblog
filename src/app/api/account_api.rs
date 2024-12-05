use crate::app::service::account::Account;
use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use validator::Validate;

#[handler]
pub async fn login(req: &mut Request) -> ApiResult<ObjResponse<String>> {
    let params = req.parse_json::<Account>().await?;
    params.validate()?;
    crate::app::service::account::login(params).await
}
