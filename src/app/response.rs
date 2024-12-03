use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ObjectResponse<'a, T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    pub status: usize,
    pub data: &'a T,
}

#[derive(Debug, Serialize)]
pub struct ListResponse<'a, T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    pub status: usize,
    pub data: &'a Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
}

