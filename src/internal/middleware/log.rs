use salvo::http::{ResBody, StatusCode};
use salvo::{async_trait, Depot, FlowCtrl, Request, Response};
use std::time::Instant;
use tracing::{Instrument, Level};

pub struct LogMiddleware {}

impl LogMiddleware {
    #[inline]
    pub fn new() -> Self {
        LogMiddleware {}
    }
}

#[async_trait]
impl salvo::Handler for LogMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let span = tracing::span!(
            Level::INFO,
            "Request",
            remote_addr = %req.remote_addr().to_string(),
            version = ?req.version(),
            method = %req.method(),
            path = %req.uri(),
        );

        async move {
            let now = Instant::now();
            ctrl.call_next(req, depot, res).await;
            let duration = now.elapsed();
            let status = res.status_code.unwrap_or(match &res.body {
                ResBody::None => StatusCode::NOT_FOUND,
                ResBody::Error(e) => e.code,
                _ => StatusCode::OK,
            });
            tracing::info!(
                %status,
                ?duration,
                "Response"
            );
        }
        .instrument(span)
        .await
    }
}
