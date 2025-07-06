use std::{error::Error, fmt::Display};

use async_trait::async_trait;
use axum::Router;

use crate::server::error::ApiServerError;

pub trait ServiceError: Display + Error + Into<ApiServerError> {}

#[async_trait]
pub trait Service {
    type Error: ServiceError;

    async fn router() -> Result<Router, Self::Error>;
}
