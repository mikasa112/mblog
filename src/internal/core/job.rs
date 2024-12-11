use crate::internal::core::my_error::MyJobError;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;
use crate::app::model;

pub struct MyJob {
    _scheduler: JobScheduler,
}

impl MyJob {
    pub async fn new() -> Result<Self, MyJobError> {
        let sched = JobScheduler::new().await?;
        let sync_job = Self::sync_database_job().await?;
        sched.add(sync_job).await?;
        sched.start().await?;
        Ok(MyJob {
            _scheduler: sched,
        })
    }

    async fn sync_database_job() -> Result<Job, MyJobError> {
        let job = Job::new_async("0/10 * * * * *", |_, _| {
            Box::pin(async move {
                let a = model::posts::PostCategory::query_posts_list(20, 0).await.unwrap();
                info!(vec=?a)
            })
        })?;
        Ok(job)
    }
}