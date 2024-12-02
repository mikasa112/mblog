use salvo::{handler, Response};
use salvo::prelude::Json;
use crate::app::response::ListResponse;

#[handler]
pub async fn list_posts(res: &mut Response) {
    res.render(Json(ListResponse {
        err_msg: None,
        status: 0,
        data: vec![1, 2, 3],
        total: None,
    }));
}

#[handler]
pub async fn one_posts(res: &mut Response) {
    res.render(Json(()))
}