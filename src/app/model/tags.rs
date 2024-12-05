use crate::internal::core::database::db_pool;
use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
}
