use std::borrow::Cow;

use axum::Json;
use axum::http::StatusCode;
use birb_common::error::{ErrorResponse, IntoErrorResponse};
use birb_macros::SerializedErrorResponse;
use sqlx::Error as SQLError;

#[derive(Debug, thiserror::Error, SerializedErrorResponse)]
pub enum SchemaError {
    #[error("Not Found")]
    NotFound,
    #[error("Fatal sqlx error: {0}")]
    Fatal(SQLError),
}

impl From<SQLError> for SchemaError {
    fn from(value: SQLError) -> Self {
        match value {
            SQLError::RowNotFound => Self::NotFound,
            err => Self::Fatal(err),
        }
    }
}

impl<'a> IntoErrorResponse<'a> for SchemaError {
    fn error(self) -> ErrorResponse<'a> {
        match self {
            SchemaError::NotFound => ErrorResponse::new(
                "NOT_FOUND",
                Cow::Borrowed("Requested content was not found."),
                StatusCode::NOT_FOUND,
            ), // maybe specify what content that was not found in the future, e.g Post not found
            SchemaError::Fatal(_) => ErrorResponse::new(
                "INTERNAL_ERROR",
                Cow::Borrowed("Internal Fatal Server Error"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
