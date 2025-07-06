use std::sync::Arc;

use crate::database::{
    databases::{connection::DatabaseConnection, error::SchemaError},
    table::SqlTable,
};

use super::tables::poststable::poststable::PostsTable;

#[derive(Clone)]
pub struct BlogSchema {
    connection: Arc<DatabaseConnection>,
}

impl BlogSchema {
    pub async fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn posts(&self) -> Result<PostsTable, SchemaError> {
        let connection = self.connection.acquire_connection().await?;

        Ok(PostsTable::new(connection).await?)
    }
}
