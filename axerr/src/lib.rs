use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    code: StatusCode,
}
