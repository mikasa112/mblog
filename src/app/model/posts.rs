use crate::internal::core::database::db_pool;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, MySql, QueryBuilder, Type};

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

#[derive(Type, Debug, Copy, Clone)]
#[sqlx(type_name = "status")] // 数据库中自定义类型的名称
#[sqlx(rename_all = "lowercase")] // 指定如何将枚举值映射到数据库
pub enum Status {
    Draft,
    Published,
}

// 避免孤儿规则
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MStatus(Option<Status>);
impl From<Option<String>> for MStatus {
    fn from(value: Option<String>) -> Self {
        match value {
            None => MStatus(None),
            Some(s) => match s.as_str() {
                "draft" => MStatus(Some(Status::Draft)),
                "published" => MStatus(Some(Status::Published)),
                _ => MStatus(None),
            },
        }
    }
}

impl Post {
    /// 查询文章列表
    pub async fn query_posts_list(limit: u32, offset: u32) -> Result<Vec<Post>, sqlx::Error> {
        let result: Vec<Post> = sqlx::query_as!(
            Post,
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
        "#,
            limit,
            offset
        )
        .fetch_all(db_pool())
        .await?;
        Ok(result)
    }

    /// 从文章ID查询文章
    pub async fn query_posts_by_id(id: u32) -> Result<Post, sqlx::Error> {
        let result = sqlx::query_as!(
            Post,
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
        "#,
            id
        )
        .fetch_one(db_pool())
        .await?;
        Ok(result)
    }

    /// 查询文章总数
    pub async fn query_posts_count() -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
        SELECT  COUNT(*) AS total FROM  t_posts tp;
        "#
        )
        .fetch_one(db_pool())
        .await?;
        Ok(result.total)
    }

    /// 插入文章
    pub async fn insert_post(
        category_id: Option<u32>,
        title: String,
        content: String,
        excerpt: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO d_blog.t_posts
        (category_id, title, content, excerpt)
        VALUES(?, ?, ?, ?);
        "#,
            category_id,
            title,
            content,
            excerpt
        )
        .execute(db_pool())
        .await?;
        Ok(())
    }

    /// 更新文章的分类标签
    pub async fn update_post_category(
        id: u32,
        category_id: Option<u32>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        UPDATE d_blog.t_posts
        SET category_id=?
        WHERE id=?;
        "#,
            category_id,
            id
        )
        .execute(db_pool())
        .await?;
        Ok(())
    }

    /// 动态生成的更新文章SQL
    pub async fn update_post(
        id: u32,
        category_id: Option<u32>,
        title: Option<String>,
        content: Option<String>,
        excerpt: Option<String>,
    ) -> Result<(), sqlx::Error> {
        let mut builder: QueryBuilder<MySql> = QueryBuilder::new("UPDATE d_blog.t_posts tt SET ");
        let mut has_update = false;
        if let Some(it) = category_id {
            if has_update {
                builder.push(", ");
            }
            builder.push("tt.category_id = ");
            builder.push_bind(it);
            has_update = true;
        }
        if let Some(it) = title {
            if has_update {
                builder.push(", ");
            }
            builder.push("tt.title = ");
            builder.push_bind(it);
            has_update = true;
        }
        if let Some(it) = content {
            if has_update {
                builder.push(", ");
            }
            builder.push("tt.content = ");
            builder.push_bind(it);
            has_update = true;
        }
        if let Some(it) = excerpt {
            if has_update {
                builder.push(", ");
            }
            builder.push("tt.excerpt = ");
            builder.push_bind(it);
            has_update = true;
        }
        if !has_update {
            return Ok(());
        }
        builder.push(" WHERE id =");
        builder.push_bind(id);
        let query = builder.build();
        query.execute(db_pool()).await?;
        Ok(())
    }

    /// 对文章绑定标签
    pub async fn insert_post_tag(post_id: u32, tag_id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
          INSERT INTO d_blog.t_post_tags (post_id, tag_id) VALUES(?, ?);
          "#,
            post_id,
            tag_id
        )
        .execute(db_pool())
        .await?;
        Ok(())
    }

    /// 删除{post_id}文章的{tag_id}标签
    pub async fn delete_post_tag(post_id: u32, tag_id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
          DELETE FROM d_blog.t_post_tags WHERE post_id = ? AND tag_id = ?;
          "#,
            post_id,
            tag_id
        )
        .execute(db_pool())
        .await?;
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

#[derive(FromRow, Debug)]
pub struct PDetail {
    pub id: u32,
    pub category_name: Option<String>,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: MStatus,
    pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PostCategory {
    pub async fn query_posts_list(
        limit: u32,
        offset: u32,
    ) -> Result<Vec<PostCategory>, sqlx::Error> {
        let list = sqlx::query_as!(
            PostCategory,
            r#"
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
        "#,
            limit,
            offset
        )
        .fetch_all(db_pool())
        .await?;
        Ok(list)
    }

    pub async fn query_posts_by_id(id: u32) -> Result<Option<PDetail>, sqlx::Error> {
        #[derive(FromRow, Debug, Clone)]
        struct Post {
            pub id: u32,
            pub category_name: Option<String>,
            pub title: Option<String>,
            pub content: Option<String>,
            pub excerpt: Option<String>,
            pub status: MStatus,
            pub tag_name: Option<String>,
            pub created_at: NaiveDateTime,
            pub updated_at: NaiveDateTime,
        }
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT
            tp.id, tc.name AS category_name, tp.title, tp.content, tp.excerpt, tp.status, tp.created_at, tp.updated_at, tt.name AS tag_name
            FROM
                d_blog.t_posts tp
            LEFT JOIN
                d_blog.t_categories tc
            ON
                tp.category_id = tc.id
            LEFT JOIN
                d_blog.t_post_tags tpt
            ON
                tp.id = tpt.post_id
            LEFT JOIN
                d_blog.t_tags tt
            ON
                tpt.tag_id = tt.id
            WHERE
                tp.id = ?;
            "#,
            id
        )
            .fetch_all(db_pool())
            .await?;
        if !posts.is_empty() {
            let p_one = posts[0].clone();
            let tags = posts.into_iter().map(|it| it.tag_name).collect();
            let p = PDetail {
                id: p_one.id,
                category_name: p_one.category_name,
                title: p_one.title.unwrap_or("".to_string()),
                content: p_one.content.unwrap_or("".to_string()),
                excerpt: p_one.excerpt,
                status: p_one.status,
                tags,
                created_at: p_one.created_at,
                updated_at: p_one.updated_at,
            };
            Ok(Some(p))
        } else {
            Ok(None)
        }
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
        Post::insert_post(
            Some(1),
            "我是一条测试标题".to_string(),
            "测试内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容内容..."
                .to_string(),
            Some(String::from("我是一条摘要")),
        )
        .await
        .unwrap();
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

    #[tokio::test]
    async fn test_update_post() {
        Post::update_post(1, Some(1), Some("标题2标题2".to_string()), None, None)
            .await
            .unwrap();
    }
}
