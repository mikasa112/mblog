use tantivy::directory::error::OpenDirectoryError;
use tantivy::query::QueryParserError;
use tantivy::TantivyError;
use tokio_cron_scheduler::JobSchedulerError;

#[derive(thiserror::Error, Debug)]
pub enum MyJobError {
    #[error("定时任务错误: {source}")]
    JobSchedulerError {
        #[from]
        source: JobSchedulerError
    }
}


#[derive(thiserror::Error, Debug)]
pub enum SearchEngineError {
    #[error("打开目录错误: {source}")]
    OpenDirectoryError {
        #[from]
        source: OpenDirectoryError
    },
    #[error("Tantivt错误: {source}")]
    TantivyError {
        #[from]
        source: TantivyError
    },
    #[error("查询参数解析错误: {source}")]
    QueryParserError {
        #[from]
        source: QueryParserError
    },
}