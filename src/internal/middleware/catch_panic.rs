use crate::internal::result::response::ObjResponse;
use futures_util::FutureExt;
use log::error;
use salvo::prelude::Json;
use salvo::{async_trait, Depot, FlowCtrl, Request, Response};
use std::panic::AssertUnwindSafe;

pub struct CatchPanic;

impl CatchPanic {
    #[inline]
    pub fn new() -> CatchPanic {
        CatchPanic
    }
}

#[async_trait]
impl salvo::Handler for CatchPanic {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        if let Err(e) = AssertUnwindSafe(ctrl.call_next(req, depot, res))
            .catch_unwind()
            .await
        {
            error!("panic occurred: {e:?}",);
            res.render(Json(ObjResponse::<()> {
                err_msg: Option::from("服务器内部错误".to_string()),
                status: 0,
                data: None,
            }))
        }
    }
}
