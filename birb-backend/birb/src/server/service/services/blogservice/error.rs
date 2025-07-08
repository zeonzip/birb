use crate::database::databases::error::SchemaError;

#[derive(Debug, thiserror::Error)]
pub enum BlogServiceError {
    #[error("{0}")]
    Schema(#[from] SchemaError),
    #[error("{0}")]
    Database(#[from] sqlx::Error),
}
