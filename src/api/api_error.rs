
use axum::response::IntoResponse;
use axum::http;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InternalError(String),
    Teapot(String),
    BadRequest(String),
    Unauthorized(String),
}

#[derive(serde::Serialize, Debug)]
pub struct AppErrorPayload {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::InternalError(msg) => {
                let body = json!( AppErrorPayload{
                    message: msg,
                });

                (   
                    http::StatusCode::INTERNAL_SERVER_ERROR, 
                    [(http::header::CONTENT_TYPE, "application/json")],
                    body.to_string()
                ).into_response()
            },

            AppError::Teapot(msg) => {
                let body = json!( AppErrorPayload{
                    message: msg,
                });

                (   
                    http::StatusCode::IM_A_TEAPOT, 
                    [(http::header::CONTENT_TYPE, "application/json")],
                    body.to_string()
                ).into_response()
            },

            AppError::BadRequest(msg) => {
                let body = json!( AppErrorPayload{
                    message: msg,
                });

                (   
                    http::StatusCode::BAD_REQUEST, 
                    [(http::header::CONTENT_TYPE, "application/json")],
                    body.to_string()
                ).into_response()
            },
            AppError::Unauthorized(msg) => {
                let body = json!( AppErrorPayload{
                    message: msg,
                });

                (   
                    http::StatusCode::UNAUTHORIZED, 
                    [(http::header::CONTENT_TYPE, "application/json")],
                    body.to_string()
                ).into_response()
            },
        }
    }
}