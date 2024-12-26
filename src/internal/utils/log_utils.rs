use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Layer};

pub struct Logger {
    _guard: WorkerGuard,
}
impl Logger {
    pub fn init() -> Logger {
        LogTracer::builder().init().unwrap();
        let fmt_layer = fmt::layer()
            .with_level(true)
            .with_writer(std::io::stdout)
            .with_filter(LevelFilter::INFO);
        let file_appender = tracing_appender::rolling::daily("./logs/", "mblog.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::layer()
            .with_ansi(false)
            .with_writer(non_blocking)
            .with_filter(LevelFilter::INFO);
        let collector = tracing_subscriber::registry()
            .with(file_layer)
            .with(fmt_layer);
        tracing::subscriber::set_global_default(collector).unwrap();
        Logger { _guard: guard }
    }
}
