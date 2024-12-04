pub mod response;
pub mod code;

pub type ApiResult<T> = Result<T, code::Code>;