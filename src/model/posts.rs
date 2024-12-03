use sqlx::{query, query_as, FromRow};
use sqlx::mysql::MySqlQueryResult;
use sqlx::types::chrono::NaiveDateTime;
use crate::app::database::{db_pool};
use crate::model::SQLError;

#[derive(Debug, FromRow)]
pub struct Posts {
    pub id: u32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Posts {
    /// 查询文章列表
    pub async fn query_posts_list(limit: u32, offset: u32) -> Result<Vec<Posts>, SQLError> {
        let result: Vec<Posts> = query_as!(Posts,
        r#"
        SELECT tp.id, tp.content ,tp.created_at,tp.updated_at FROM t_posts tp ORDER BY tp.updated_at DESC LIMIT ? OFFSET ?;
        "#,limit ,offset).fetch_all(db_pool()).await?;
        Ok(result)
    }

    /// 从文章ID查询文章
    pub async fn query_posts_by_id(id: u32) -> Result<Posts, SQLError> {
        let result = query_as!(Posts,
            r#"
        SELECT tp.id, tp.content,tp.created_at,tp.updated_at FROM  t_posts tp WHERE tp.id = ?;
        "#,id).fetch_one(db_pool()).await?;
        Ok(result)
    }

    /// 查询文章总数
    pub async fn query_posts_count() -> Result<i64, SQLError> {
        let result = query!(r#"
        SELECT  COUNT(*) AS total FROM  t_posts tp;
        "#).fetch_one(db_pool()).await?;
        Ok(result.total)
    }
}
#[cfg(test)]
mod posts_test {
    use super::*;

    #[tokio::test]
    async fn query_posts_by_id() {
        let a = Posts::query_posts_by_id(2).await.unwrap();
        assert_eq!(a.content, "测试测试测试")
    }

    #[tokio::test]
    async fn test_query_posts_list() {
        let posts = Posts::query_posts_list(10, 0).await.unwrap();
        assert!(posts.len() <= 10);
    }

    #[tokio::test]
    async fn test_query_count() {
        let result = Posts::query_posts_count().await.unwrap();
        assert!(result > 0)
    }
}