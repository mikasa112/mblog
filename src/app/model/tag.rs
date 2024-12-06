use crate::internal::core::database::db_pool;
use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub struct TagPost {
    pub id: u32,
    pub name: String,
    pub post_id: u32,
    pub post_title: String,
}


impl Tag {
    pub async fn list() -> Result<Vec<Self>, sqlx::Error> {
        let list = sqlx::query_as!(
            Tag,
            r#"
            SELECT
            tt.id,
            tt.name,
            tt.created_at,
            tt.updated_at  FROM  t_tags tt
            ORDER BY updated_at DESC;
            "#
        )
            .fetch_all(db_pool())
            .await?;
        Ok(list)
    }

    pub async fn create(name: String) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"
        INSERT INTO d_blog.t_tags
        (name)
        VALUES(?);
        "#,name).execute(db_pool()).await?;
        Ok(())
    }

    pub async fn delete(id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"
        DELETE FROM d_blog.t_tags WHERE id = ?;
        "#,id).execute(db_pool()).await?;
        Ok(())
    }
}
