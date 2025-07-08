use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use birb_common::error::ErrorResponse;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use super::{
    error::ApiServerError,
    service::{service::Service, services::blogservice::blogservice::BlogService},
};

pub struct ApiServer {
    listener: TcpListener,
}

impl ApiServer {
    pub async fn connect() -> Result<Self, ApiServerError> {
        let listener = TcpListener::bind("0.0.0.0:7853").await?;

        Ok(Self { listener })
    }

    pub async fn run(self) -> Result<(), ApiServerError> {
        let cors = CorsLayer::permissive();

        let app = Router::new()
            .merge(BlogService::router().await?)
            .route("/{*wildcard}", get(wildcard))
            .layer(cors);

        axum::serve(self.listener, app).await?;

        Ok(())
    }
}

async fn wildcard(Path(path): Path<String>) -> impl IntoResponse {
    let wildcard_response: ErrorResponse = ErrorResponse::new(
        "ROUTE_NOT_FOUND",
        format!("{path} isn't a valid route!").into(),
        StatusCode::NOT_FOUND,
    );

    wildcard_response.response()
}
