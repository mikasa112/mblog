use crate::app::service;
use crate::internal::result::response::{ListResponse, ObjResponse};
use crate::internal::result::ApiResult;
use salvo::{handler, Request};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[handler]
pub async fn categories() -> ApiResult<ListResponse<service::category::CategorySimpler>> {
    service::category::list().await
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CategoryParams {
    #[validate(length(min = 1, message = "name不能为空"))]
    pub name: String,
    pub description: Option<String>,
}

#[handler]
pub async fn create_category(req: &mut Request) -> ApiResult<ObjResponse<()>> {
    let params = req.parse_json::<CategoryParams>().await?;
    params.validate()?;
    service::category::create(params).await
}
