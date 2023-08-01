use std::io::Result;
use actix_web::{http::StatusCode, HttpResponse};
use crate::dto::error::ErrorResponse;

pub fn http_error(code: StatusCode, message: &str) -> Result<HttpResponse> {
    Ok(HttpResponse::build(code).json(ErrorResponse {
        code: code.as_u16(),
        message: message.to_string(),
    }))
}