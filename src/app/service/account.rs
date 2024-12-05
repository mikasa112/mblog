use jsonwebtoken::{EncodingKey, Header};
use salvo::http::cookie::time::{Duration, OffsetDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::internal::middleware::auth::JwtClaims;
use crate::internal::result::ApiResult;
use crate::internal::result::code::Code;
use crate::internal::result::response::ObjResponse;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Account {
    #[validate(length(min = 1, message = "UserName不能为空"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password不能为空"))]
    pub password: String,
}

pub async fn login(params: Account) -> ApiResult<ObjResponse<String>> {
    let exp = OffsetDateTime::now_utc() + Duration::days(1);
    let claims = JwtClaims {
        username: params.username,
        exp: exp.unix_timestamp(),
    };
    let token = jsonwebtoken::encode(&Header::default(),
                                     &claims,
                                     &EncodingKey::from_secret(crate::internal::core::config::BLOG_CONFIG.application.secret_key.as_bytes())).map_err(|e| {
        return Code::New(99999, String::from("TOKEN生成失败"));
    })?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: Some(token),
    })
}