use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub desc: String,
    #[serde(default)]
    pub payload: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BasicResponse {
    #[serde(default)]
    pub code: i32,
    #[serde(default)]
    pub desc: String,
}
