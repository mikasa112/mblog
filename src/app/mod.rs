use salvo::{Depot, Request, Response, Writer};
use thiserror::Error;
use tracing::log::{log, Level};

pub mod database;
pub mod response;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError()
}

// impl Writer for AppError {
//     async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
//         todo!()
//     }
// }

pub type AppResult<T> = Result<T, AppError>;