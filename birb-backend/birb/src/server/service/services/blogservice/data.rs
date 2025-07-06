use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::prelude::FromRow;

pub type BlogId = i32;

#[derive(Serialize, Deserialize)]
pub struct NewBlogPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdResponse {
    pub id: BlogId,
}

#[derive(FromRow, Serialize)]
pub struct BlogPost {
    pub id: BlogId,
    pub title: String,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl BlogPost {
    pub fn response(self) -> Json<Self> {
        Json(self)
    }
}
