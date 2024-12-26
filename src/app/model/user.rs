use crate::internal::core::database::db_pool;
use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub passw: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn query_user(username: String) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            Self,
            r#"
        SELECT id, username, passw, created_at,updated_at FROM d_blog.t_user WHERE username = ?;
        "#,
            username
        )
        .fetch_one(db_pool())
        .await?;
        Ok(user)
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct UserInfo {
    pub nick_name: Option<String>,
    pub info: Option<String>,
}

impl UserInfo {
    pub async fn query_user(username: &str) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            Self,
            r#"
            SELECT nick_name, info FROM d_blog.t_user WHERE username = ?;
            "#,
            username
        )
        .fetch_one(db_pool())
        .await?;
        Ok(user)
    }
}

mod test {
    #[tokio::test]
    async fn test_query_user() {
        let u = crate::app::model::user::User::query_user(String::from("mikasa"))
            .await
            .unwrap();
        println!("{:?}", u);
    }
}
