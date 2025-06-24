use crate::app::model::user;
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
pub struct AccountParams {
    #[validate(length(min = 1, message = "UserName不能为空"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password不能为空"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}

pub async fn login(params: AccountParams) -> ApiResult<ObjResponse<String>> {
    //查询用户
    let user = user::User::query_user(params.username)
        .await
        .map_err(|_| Code::New(99999, "用户名或者密码错误".to_string()))?;
    let argon2_result =
        tokio::task::spawn_blocking(move || match PasswordHash::new(user.passw.as_str()) {
            Ok(password_hash) => {
                match argon2::Argon2::default()
                    .verify_password(params.password.as_bytes(), &password_hash)
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Code::New(99999, "用户名或者密码错误".to_string())),
                }
            }
            Err(_) => Err(Code::New(99998, "密码HASH失败".to_string())),
        })
        .await
        .map_err(|e| {
            log::error!("tokio runtime 错误: {}", e);
            Code::New(99999, "系统内部错误".to_string())
        })?;
    //校验密码
    match argon2_result {
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
            .map_err(|_e| Code::New(99997, String::from("TOKEN生成失败")))?;
            Ok(ObjResponse {
                err_msg: None,
                status: 0,
                data: Some(token),
            })
        }
        Err(e) => Err(e),
    }
}

pub async fn user_info() -> ApiResult<ObjResponse<UserInfo>> {
    let user = user::UserInfo::query_user("mikasa").await?;
    Ok(ObjResponse {
        err_msg: None,
        status: 0,
        data: Some(UserInfo {
            nick_name: user.nick_name,
            info: user.info,
        }),
    })
}
