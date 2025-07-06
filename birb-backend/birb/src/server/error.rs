use super::service::services::blogservice::error::BlogServiceError;

#[derive(Debug, thiserror::Error)]
pub enum ApiServerError {
    #[error("{0}")]
    BlogServiceErr(#[from] BlogServiceError),
    #[error("{0}")]
    Io(#[from] tokio::io::Error),
}
