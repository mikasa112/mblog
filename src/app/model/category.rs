use crate::internal::core::database::db_pool;
use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Category {
    /// 查询所有分类
    pub async fn list() -> Result<Vec<Self>, sqlx::Error> {
        let categories = sqlx::query_as!(
            Category,
            r#"
        SELECT
        tc.id,
        tc.name,
        tc.description,
        tc.created_at,
        tc.updated_at
        FROM  t_categories tc;
        "#
        )
        .fetch_all(db_pool())
        .await?;
        Ok(categories)
    }

    /// 创建一条分类
    pub async fn create(name: String, description: Option<String>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO d_blog.t_categories
        (name, description)
        VALUES(?, ?);
        "#,
            name,
            description
        )
        .execute(db_pool())
        .await?;
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct CategorySimpler {
    pub id: u32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub post_count: i64,
}

impl CategorySimpler {
    /// 按时间降序查询分类列表以及分类列表下的文章总数
    pub async fn list() -> Result<Vec<CategorySimpler>, sqlx::Error> {
        let list = sqlx::query_as!(
            CategorySimpler,
            r#"
           SELECT
             tc.id,
             tc.name,
             tc.description,
             COUNT(tp.id) AS post_count
         FROM
             d_blog.t_categories tc
         LEFT JOIN
             d_blog.t_posts tp
         ON
             tc.id = tp.category_id
         GROUP BY
             tc.id, tc.name, tc.description
         ORDER BY
             tc.updated_at DESC;
        "#
        )
        .fetch_all(db_pool())
        .await?;
        Ok(list)
    }

    pub async fn count() -> Result<usize, sqlx::Error> {
        let result = sqlx::query!(r#"SELECT  COUNT(*) AS total FROM  d_blog.t_categories tc;"#)
            .fetch_one(db_pool())
            .await?;
        Ok(result.total as usize)
    }
}

mod test {
    #[tokio::test]
    async fn test_create() {
        crate::app::model::category::Category::create(
            String::from("算法"),
            Some(String::from("算法学习")),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_list() {
        let list = crate::app::model::category::Category::list().await.unwrap();
        println!("{:?}", list);
        assert!(list.len() > 0);
    }
}
