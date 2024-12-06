use crate::internal::core::database::db_pool;
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

    // pub async fn query_category_posts()
}

mod test {
    // #[tokio::test]
    // async fn test_create() {
    //     Categories::create(String::from("算法"), Some(String::from("算法学习"))).await.unwrap();
    // }

    #[tokio::test]
    async fn test_list() {
        let list = crate::app::model::category::Category::list().await.unwrap();
        println!("{:?}", list);
        assert!(list.len() > 0);
    }
}
