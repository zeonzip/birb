use std::sync::Arc;

use crate::database::databases::{connection::DatabaseConnection, error::SchemaError};

#[derive(Clone)]
pub struct AuthSchema {
    connection: Arc<DatabaseConnection>,
}

impl AuthSchema {
    pub async fn new(connection: Arc<DatabaseConnection>) -> Result<Self, SchemaError> {
        Ok(Self { connection })
    }
}
