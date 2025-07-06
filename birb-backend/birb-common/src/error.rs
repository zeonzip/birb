use std::borrow::Cow;

use axum::{Json, http::StatusCode};
use serde::Serialize;

pub type ErrorResponseSer<'a> = (StatusCode, Json<ErrorResponse<'a>>);

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    error: &'static str,
    message: Cow<'a, str>,
    #[serde(skip_serializing)]
    code: StatusCode,
}

impl<'a> ErrorResponse<'a> {
    pub fn new(error: &'static str, message: Cow<'a, str>, code: StatusCode) -> Self {
        Self {
            error,
            message,
            code,
        }
    }

    pub fn response(self) -> ErrorResponseSer<'a> {
        (self.code, Json(self))
    }
}

pub trait IntoErrorResponse<'a> {
    fn error(self) -> ErrorResponse<'a>;
}
