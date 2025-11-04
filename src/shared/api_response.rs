use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: String,
    pub ts: i64,
}

impl<T : Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            msg: "success".to_string(),
            ts: Utc::now().timestamp_millis(),
        }
    }

    pub fn success_with_msg(data: T, msg: &str) -> Self {
        Self {
            code: 0,
            data: Some(data),
            msg: msg.to_string(),
            ts: Utc::now().timestamp_millis(),
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            code: -1,
            data: None,
            msg: msg.to_string(),
            ts: Utc::now().timestamp_millis(),
        }
    }

    pub fn error_with_code(code: i32, msg: &str) -> Self {
        Self {
            code,
            data: None,
            msg: msg.to_string(),
            ts: Utc::now().timestamp_millis(),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.code == 0 {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        (status, Json(self)).into_response()
    }
}

pub async fn handle_error(error: impl Into<anyhow::Error>) -> Response {
    let error = error.into();
    let response = ApiResponse::<()>::error_with_code(-1, &error.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
}

