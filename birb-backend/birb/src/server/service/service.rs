use std::{error::Error, fmt::Display};

use async_trait::async_trait;
use axum::Router;

use crate::server::error::ApiServerError;

#[async_trait]
pub trait Service {
    type Error: Display + Error + Into<ApiServerError>;

    async fn router() -> Result<Router, Self::Error>;
}
