use crate::internal::core::config::BLOG_CONFIG;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::JwtAuth;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    username: String,
    exp: i64,
}

#[inline]
pub fn auth_handler() -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(BLOG_CONFIG.application.secret_key.as_bytes()))
        .finders(vec![
            Box::new(HeaderFinder::new()),
        ])
        .force_passed(false)
}

