use sqlx::{FromRow, Type};
use sqlx::types::chrono::NaiveDateTime;
use crate::internal::core::database::db_pool;

#[derive(FromRow, Debug)]
pub struct Post {
    pub id: u32,
    pub category_id: Option<u32>,
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
#[warn(dead_code)]
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

impl Post {
    /// 查询文章列表
    pub async fn query_posts_list(limit: u32, offset: u32) -> Result<Vec<Post>, sqlx::Error> {
        let result: Vec<Post> = sqlx::query_as!(Post,
        r#"
        SELECT
        tp.id,
        tp.category_id,
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
    pub async fn query_posts_by_id(id: u32) -> Result<Post, sqlx::Error> {
        let result = sqlx::query_as!(Post,
        r#"
        SELECT
        tp.id,
        tp.category_id,
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
        let result = sqlx::query!(r#"
        SELECT  COUNT(*) AS total FROM  t_posts tp;
        "#).fetch_one(db_pool()).await?;
        Ok(result.total)
    }

    /// 插入文章
    pub async fn insert_post(category_id: Option<u32>, title: String, content: String, excerpt: Option<String>) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"
        INSERT INTO d_blog.t_posts
        (category_id, title, content, excerpt)
        VALUES(?, ?, ?, ?);
        "#,category_id, title, content, excerpt).execute(db_pool()).await?;
        Ok(())
    }

    /// 更新文章的分类标签
    pub async fn update_post_category(id: u32, category_id: Option<u32>) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"
        UPDATE d_blog.t_posts
        SET category_id=?
        WHERE id=?;
        "#,category_id, id).execute(db_pool()).await?;
        Ok(())
    }
}

#[derive(FromRow, Debug)]
pub struct PostCategory {
    pub id: u32,
    pub category_name: Option<String>,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: MStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PostCategory {
    pub async fn query_posts_list(limit: u32, offset: u32) -> Result<Vec<PostCategory>, sqlx::Error> {
        let list = sqlx::query_as!(PostCategory,r#"
            SELECT
            tp.id,
            tc.name AS category_name,
            tp.title,
            tp.content,
            tp.excerpt,
            tp.status,
            tp.created_at,
            tp.updated_at
            FROM t_posts tp LEFT JOIN t_categories tc ON tp.category_id = tc.id
            ORDER BY tp.updated_at DESC LIMIT ? OFFSET ?;
        "#, limit, offset).fetch_all(db_pool()).await?;
        Ok(list)
    }

    pub async fn query_posts_by_id(id: u32) -> Result<PostCategory, sqlx::Error> {
        let post_category = sqlx::query_as!(PostCategory,r#"
            SELECT
            tp.id,
            tc.name AS category_name,
            tp.title,
            tp.content,
            tp.excerpt,
            tp.status,
            tp.created_at,
            tp.updated_at
            FROM t_posts tp LEFT JOIN t_categories tc ON tp.category_id = tc.id
            WHERE tp.id = ?;
        "#,id).fetch_one(db_pool()).await?;
        Ok(post_category)
    }
}

#[cfg(test)]
mod posts_test {
    use super::*;

    #[tokio::test]
    async fn query_posts_by_id() {
        let a = Post::query_posts_by_id(2).await.unwrap();
        assert_eq!(a.content, "测试测试测试")
    }

    #[tokio::test]
    async fn test_query_posts_list() {
        let posts = Post::query_posts_list(10, 0).await.unwrap();
        assert!(posts.len() <= 10);
    }

    #[tokio::test]
    async fn test_query_count() {
        let result = Post::query_posts_count().await.unwrap();
        assert!(result > 0)
    }

    #[tokio::test]
    async fn test_insert_one() {
        Post::insert_post(Some(1), "我是一条测试标题".to_string(), "测试内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容...".to_string(), Some(String::from("我是一条摘要"))).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_post_category() {
        Post::update_post_category(1, None).await.unwrap();
    }

    #[tokio::test]
    async fn test_post_category_list() {
        let posts = PostCategory::query_posts_list(10, 0).await.unwrap();
        println!("{:?}", posts);
        assert!(posts.len() <= 10);
    }
}