use salvo::prelude::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ObjResponse<T>
where
    T: Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse<T>
where
    T: Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
}

#[async_trait]
impl<T> Writer for ObjResponse<T>
where
    T: Serialize + Send,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self))
    }
}
#[async_trait]
impl<T> Writer for ListResponse<T>
where
    T: Serialize + Send,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self))
    }
}
