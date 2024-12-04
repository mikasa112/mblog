use salvo::{async_trait, Depot, Request, Response, Writer};
use salvo::prelude::Json;
use thiserror::Error;
use sqlx::Error;
use crate::internal::result::response::ObjResponse;

#[derive(Error, Debug)]
pub enum Code {
    #[error("{1}: {0}")]
    New(i32, String),
    #[error("内部服务器错误: {source}")]
    InternalServerError {
        #[from]
        source: Error,
    },
}

#[async_trait]
impl Writer for Code {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let (code, msg) = match self {
            Code::New(code, msg) => (code, Some(msg)),
            Code::InternalServerError { source } => {
                (10000, Some(source.to_string()))
            }
        };
        res.render(Json(ObjResponse::<()> {
            err_msg: msg,
            status: code,
            data: None,
        }))
    }
}