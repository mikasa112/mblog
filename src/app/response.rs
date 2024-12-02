use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ObjectResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    pub status: usize,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    pub status: usize,
    pub data: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
}

