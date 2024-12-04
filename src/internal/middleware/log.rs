use salvo::{async_trait, Depot, FlowCtrl, Request, Response};
use tracing::info;

pub struct LogMiddleware {}

impl LogMiddleware {
    #[inline]
    pub fn new() -> Self {
        LogMiddleware {}
    }
}

#[async_trait]
impl salvo::Handler for LogMiddleware {
    async fn handle(&self, req: &mut Request, _depot: &mut Depot, _res: &mut Response, _ctrl: &mut FlowCtrl) {
        let req_method = req.method().to_string();
        let req_uri = req.uri().to_string();
        info!("{} {}", req_method, req_uri);
    }
}