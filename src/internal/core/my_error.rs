use tokio_cron_scheduler::JobSchedulerError;

#[derive(thiserror::Error, Debug)]
pub enum MyJobError {
    #[error("定时任务错误: {source}")]
    JobSchedulerError {
        #[from]
        source: JobSchedulerError
    }
}