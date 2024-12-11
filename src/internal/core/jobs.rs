use crate::internal::core::my_error::MyJobError;
use crate::internal::core::tantivy_engine::SEARCH_ENGINE;
use tantivy::doc;
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct MyJob {
    _scheduler: JobScheduler,
}

impl MyJob {
    pub async fn new() -> Result<Self, MyJobError> {
        let sched = JobScheduler::new().await?;
        let sync_job = Self::sync_database_job().await?;
        sched.add(sync_job).await?;
        sched.start().await?;
        Ok(MyJob { _scheduler: sched })
    }

    async fn sync_database_job() -> Result<Job, MyJobError> {
        let job = Job::new_async("0/10 * * * * *", |_, _| {
            Box::pin(async move { todo!("暂未有什么实现") })
        })?;
        Ok(job)
    }
}

pub struct AsyncDatabaseJob;

impl AsyncDatabaseJob {
    pub fn async_search_engine() {
        tokio::spawn(async move {
            if let Ok(engine) = &*SEARCH_ENGINE {
                let mut writer = engine.index.writer(50_000_000).unwrap();
                let id = engine.index.schema().get_field("id").unwrap();
                let title = engine.index.schema().get_field("title").unwrap();
                let content = engine.index.schema().get_field("content").unwrap();
                let excerpt = engine.index.schema().get_field("excerpt").unwrap();
                let list = crate::app::model::posts::PostCategory::query_posts_list(50, 0)
                    .await
                    .unwrap();
                for i in list.into_iter() {
                    writer
                        .add_document(doc!(
                            id=>i.id as u64,
                            title=>i.title,
                            content=>i.content,
                            excerpt=>i.excerpt.unwrap_or("".to_string())
                        ))
                        .unwrap();
                }
                writer.commit().unwrap();
            }
        });
    }
}
