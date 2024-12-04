use sqlx::{query, query_as, FromRow, Type};
use sqlx::types::chrono::NaiveDateTime;
use crate::internal::core::database::db_pool;

#[derive(FromRow, Debug)]
pub struct Posts {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: MStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Type, Debug)]
#[sqlx(type_name = "status")] // 数据库中自定义类型的名称
#[sqlx(rename_all = "lowercase")] // 指定如何将枚举值映射到数据库
pub enum Status {
    Draft,
    Published,
}


// 避免孤儿规则
#[derive(Debug)]
pub struct MStatus(Option<Status>);
impl From<Option<String>> for MStatus {
    fn from(value: Option<String>) -> Self {
        match value {
            None => MStatus(None),
            Some(s) => {
                match s.as_str() {
                    "draft" => MStatus(Some(Status::Draft)),
                    "published" => MStatus(Some(Status::Published)),
                    _ => MStatus(None),
                }
            }
        }
    }
}

impl Posts {
    /// 查询文章列表
    pub async fn query_posts_list(limit: u32, offset: u32) -> Result<Vec<Posts>, sqlx::Error> {
        let result: Vec<Posts> = query_as!(Posts,
        r#"
        SELECT
        tp.id,
        tp.title,
        tp.content,
        tp.excerpt,
        tp.status,
        tp.created_at,
        tp.updated_at
        FROM t_posts tp
        ORDER BY tp.updated_at DESC
        LIMIT ? OFFSET ?;
        "#,limit ,offset).fetch_all(db_pool()).await?;
        Ok(result)
    }

    /// 从文章ID查询文章
    pub async fn query_posts_by_id(id: u32) -> Result<Posts, sqlx::Error> {
        let result = query_as!(Posts,
        r#"
        SELECT
        tp.id,
        tp.title,
        tp.content,
        tp.excerpt,
        tp.status,
        tp.created_at,
        tp.updated_at
        FROM t_posts tp
        WHERE tp.id = ?;
        "#,id).fetch_one(db_pool()).await?;
        Ok(result)
    }

    /// 查询文章总数
    pub async fn query_posts_count() -> Result<i64, sqlx::Error> {
        let result = query!(r#"
        SELECT  COUNT(*) AS total FROM  t_posts tp;
        "#).fetch_one(db_pool()).await?;
        Ok(result.total)
    }

    /// 插入文章
    pub async fn insert_post(title: String, content: String, excerpt: Option<String>) -> Result<(), sqlx::Error> {
        query!(r#"
        INSERT INTO d_blog.t_posts
        (title, content, excerpt)
        VALUES(?, ?, ?);
        "#, title, content, excerpt).execute(db_pool()).await?;
        Ok(())
    }
}
#[cfg(test)]
mod posts_test {
    use serde::de::Unexpected::Option;
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

    // #[tokio::test]
    // async fn test_insert_one() {
    //     Posts::insert_post("我是一条测试标题".to_string(), "测试内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容...".to_string(), Some(String::from("我是一条摘要"))).await.unwrap();
    // }
}