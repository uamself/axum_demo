use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct GlobalError {
    code: u16,
    // 提示信息
    msg: String,
    // 错误信息
    error: String,
}

#[allow(dead_code)]
impl GlobalError {
    pub fn new(code: u16, msg: &str, error: &str) -> GlobalError {
        GlobalError {
            code,
            msg: String::from(msg),
            error: String::from(error),
        }
    }

    pub fn bad_request(msg: &str, error: &str) -> GlobalError {
        GlobalError::new(StatusCode::BAD_REQUEST.as_u16(), msg, error)
    }
}
