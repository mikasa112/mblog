use crate::internal::result::response::ObjResponse;
use salvo::prelude::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};

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
    #[error("参数解析错误")]
    SimpleParamsError,
}

#[async_trait]
impl Writer for Code {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let (code, msg) = match self {
            Code::New(code, msg) => (code, Some(msg)),
            Code::InternalServerError { .. } => (10000, Some(self.to_string())),
            Code::ParamsError { .. } => (10001, Some(self.to_string())),
            Code::ValidationError { validation_error } => {
                let x = validation_error
                    .field_errors()
                    .into_iter()
                    .flat_map(|(_, errors)| {
                        errors.into_iter().map(|error| {
                            format!(
                                "{}",
                                error
                                    .message
                                    .clone()
                                    .unwrap_or_else(|| "未知错误".to_string().into())
                            )
                        })
                    })
                    .collect::<Vec<String>>()
                    .join("; ");
                (10002, Some(x))
            }
            Code::SimpleParamsError => (10003, Some(self.to_string())),
        };
        res.render(Json(ObjResponse::<()> {
            err_msg: msg,
            status: code,
            data: None,
        }))
    }
}
