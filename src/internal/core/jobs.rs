use crate::app::model::posts::PostCategory;
use crate::internal::core::my_error::MyJobError;
use crate::internal::core::tantivy_engine::{PostDocument, SearchEngine, SEARCH_ENGINE};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;
use tracing::log::error;

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
            let engine = SEARCH_ENGINE
                .get_or_init(|| SearchEngine::new().expect("SearchEngine should be initialized"));
            if let Ok(count) = crate::app::model::posts::Post::query_posts_count().await {
                let mut page = 0u32;
                let size = 50;
                let mut _total = 0u32;
                while _total < count as u32 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    match PostCategory::query_posts_list(size, page * size).await {
                        Ok(list) => {
                            match engine.insert_batch(
                                list.into_iter()
                                    .map(|it| PostDocument {
                                        id: it.id as u64,
                                        title: it.title,
                                        content: it.content,
                                        excerpt: it.excerpt.unwrap_or_default(),
                                    })
                                    .collect::<Vec<PostDocument>>(),
                            ) {
                                Ok(_) => {
                                    page += 1;
                                    _total += size;
                                    info!("【同步数据到搜索引擎成功】{}页", page)
                                }
                                Err(e) => {
                                    error!("【搜索引擎错误】: {}", e.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            error!("【读取文章列表失败】: {}", e.to_string());
                        }
                    }
                }
            }
        });
    }
}
