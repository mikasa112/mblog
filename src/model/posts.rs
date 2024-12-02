use serde_yml::modules::error::Pos;
use sqlx::{query_as, FromRow};
use sqlx::types::chrono::NaiveDateTime;
use crate::app::database::{db_pool, DB_POOL};
use crate::model::ModelError;

#[derive(Debug, FromRow)]
pub struct Posts {
    pub id: u32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Posts {
    pub async fn query_posts_list(limit: i32, offset: i32) -> Result<Vec<Posts>, ModelError> {
        let result: Vec<Posts> = query_as!(Posts,
        r#"
        SELECT tp.id, tp.content ,tp.created_at,tp.updated_at FROM t_posts tp ORDER BY tp.updated_at DESC LIMIT ? OFFSET ?;
        "#,limit ,offset).fetch_all(db_pool()).await?;
        Ok(result)
    }

    pub async fn query_posts_by_id(id: u32) -> Result<Posts, ModelError> {
        let result = query_as!(Posts,
            r#"
        SELECT tp.id, tp.content ,tp.created_at,tp.updated_at from t_posts tp;
        "#).fetch_one(db_pool()).await?;
        Ok(result)
    }
}
mod posts_test {
    use crate::app::database::init_db;
    use super::*;

    #[tokio::test]
    async fn query_posts_by_id() {
        init_db().await;
        let a = Posts::query_posts_by_id(1).await.unwrap();
        println!("{:?}", a);
    }

    #[tokio::test]
    async fn test_query_posts_list() {
        init_db().await;
        let a = Posts::query_posts_list(10, 0).await.unwrap();
        println!("{:?}", a);
    }
}