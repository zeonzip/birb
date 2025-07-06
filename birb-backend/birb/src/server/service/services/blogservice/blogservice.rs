use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use birb_common::error::ErrorResponse;

use crate::{
    database::{
        databases::{blogschema::schema::BlogSchema, connection::DatabaseConnection},
        table::SqlTable,
    },
    server::service::service::Service,
};

use super::{
    data::{BlogPost, IdResponse, NewBlogPost},
    error::BlogServiceError,
};

#[derive(Clone)]
pub struct BlogService {
    database: BlogSchema,
}

#[async_trait]
impl Service for BlogService {
    type Error = BlogServiceError;

    async fn router() -> Result<axum::Router, Self::Error> {
        let state = Arc::new(Self::new().await?);

        Ok(Router::new()
            .route("/blog/{id}", get(get_blog))
            .route("/publish", post(post_blog))
            .route("/blogs", get(get_blogs))
            .with_state(state))
    }
}

impl BlogService {
    async fn new() -> Result<Self, BlogServiceError> {
        let conn = Arc::new(DatabaseConnection::connect().await?);

        let db = BlogSchema::new(conn.clone()).await;

        Ok(Self { database: db })
    }
}

async fn get_blog<'a>(
    Path(id): Path<i32>,
    State(state): State<Arc<BlogService>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse<'a>>)> {
    let mut table = state.database.posts().await?;

    let record = table.get(id).await?;

    Ok(record.response())
}

async fn post_blog<'a>(
    State(state): State<Arc<BlogService>>,
    Json(post): Json<NewBlogPost>,
) -> Result<Json<IdResponse>, (StatusCode, Json<ErrorResponse<'a>>)> {
    let mut table = state.database.posts().await?;

    let id = table.insert(post).await?;

    Ok(Json(IdResponse { id }))
}

async fn get_blogs<'a>(
    State(state): State<Arc<BlogService>>,
) -> Result<Json<Vec<BlogPost>>, (StatusCode, Json<ErrorResponse<'a>>)> {
    let mut table = state.database.posts().await?;

    let posts = table.get_all().await?;

    Ok(Json(posts))
}
