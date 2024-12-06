use crate::internal::middleware::auth::JwtClaims;
use crate::internal::result::code::Code;
use crate::internal::result::response::ObjResponse;
use crate::internal::result::ApiResult;
use argon2::{PasswordHash, PasswordVerifier};
use jsonwebtoken::{EncodingKey, Header};
use salvo::http::cookie::time::{Duration, OffsetDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Account {
    #[validate(length(min = 1, message = "UserName不能为空"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password不能为空"))]
    pub password: String,
}

pub async fn login(params: Account) -> ApiResult<ObjResponse<String>> {
    let e = Code::New(99999, "用户名或者密码错误".to_string());
    //查询用户
    let user = crate::app::model::user::User::query_user(params.username)
        .await
        .map_err(|_| Code::New(99999, "用户名或者密码错误".to_string()))?;
    let hash_pwd = PasswordHash::new(user.passw.as_str())
        .map_err(|_e| Code::New(99998, "密码HASH失败".to_string()))?;
    //校验密码
    match argon2::Argon2::default().verify_password(params.password.as_bytes(), &hash_pwd) {
        Ok(_) => {
            //生成TOKEN, 有效期为1天
            let exp = OffsetDateTime::now_utc() + Duration::days(1);
            let claims = JwtClaims {
                username: user.username,
                exp: exp.unix_timestamp(),
            };
            let token = jsonwebtoken::encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(
                    crate::internal::core::config::BLOG_CONFIG
                        .application
                        .secret_key
                        .as_bytes(),
                ),
            )
            .map_err(|_e| {
                return Code::New(99997, String::from("TOKEN生成失败"));
            })?;
            Ok(ObjResponse {
                err_msg: None,
                status: 0,
                data: Some(token),
            })
        }
        Err(_) => Err(e),
    }
}
