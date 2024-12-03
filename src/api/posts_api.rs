use salvo::{handler, Request, Response};
use salvo::prelude::{Json, StatusCode};
use serde::Serialize;
use crate::app::response::{ListResponse, ObjectResponse};
use crate::model;

#[derive(Debug, Serialize)]
struct Posts {
    pub id: u32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}


#[handler]
pub async fn list_posts(req: &mut Request, res: &mut Response) {
    let page = req.query::<u32>("page").unwrap_or(1);
    let size = req.query::<u32>("size").unwrap_or(10);
    let list = model::posts::Posts::query_posts_list(size, (page - 1) * size).await?;
    let total = model::posts::Posts::query_posts_count().await.unwrap();
    let list = list.into_iter().map(|it| {
        Posts {
            id: it.id,
            content: it.content,
            created_at: format!("{}", it.created_at.format("%Y-%m-%d %H:%M:%S")),
            updated_at: format!("{}", it.updated_at.format("%Y-%m-%d %H:%M:%S")),
        }
    }).collect::<Vec<Posts>>();
    res.render(Json(ListResponse {
        err_msg: None,
        status: 0,
        data: &list,
        total: Some(total as usize),
    }));
}

#[handler]
pub async fn one_posts(res: &mut Response) {
    let it = model::posts::Posts::query_posts_by_id(1).await.unwrap();
    let it = Posts {
        id: it.id,
        content: it.content,
        created_at: format!("{}", it.created_at.format("%Y-%m-%d %H:%M:%S")),
        updated_at: format!("{}", it.updated_at.format("%Y-%m-%d %H:%M:%S")),
    };
    res.render(Json(ObjectResponse {
        err_msg: None,
        status: 0,
        data: &it,
    }))
}