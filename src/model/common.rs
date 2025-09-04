use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CommonRequest<T> { 
    pub token: Option<String>,
    pub data: T,
}

#[derive(Deserialize, Serialize)]
pub struct CommonResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> CommonResponse<T> {
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
}

impl<T> CommonResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::new(0, "success".to_string(), Some(data))
    }
}

impl<T> CommonResponse<T> {
    pub fn err(msg: String) -> Self {
        Self::new(1, msg, None)
    }
}
