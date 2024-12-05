use salvo::{async_trait, Depot, Request, Response, Writer};
use salvo::prelude::Json;
use tracing::info;
use crate::internal::result::response::ObjResponse;

#[derive(thiserror::Error, Debug)]
pub enum Code {
    #[error("{1}: {0}")]
    New(i32, String),
    #[error("内部服务器错误: {source}")]
    InternalServerError {
        #[from]
        source: sqlx::Error,
    },
    #[error("参数解析错误: {source}")]
    ParamsError {
        #[from]
        source: salvo::http::ParseError,
    },
    #[error("参数校验错误: {validation_error}")]
    ValidationError {
        #[from]
        validation_error: validator::ValidationErrors,
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
            Code::ParamsError { source } => {
                (10001, Some(source.to_string()))
            }
            Code::ValidationError { validation_error } => {
                // let detailed_msg = format!(
                //     "参数校验失败: 字段 `{}` 的错误是 `{}`",
                //     validation_error.code,
                //     validation_error.message.unwrap_or_else(|| "未知错误".to_string().into())
                // );
                info!("{:?}",validation_error);
                (10002, Some("ValidationError".to_string()))
            }
        };
        res.render(Json(ObjResponse::<()> {
            err_msg: msg,
            status: code,
            data: None,
        }))
    }
}