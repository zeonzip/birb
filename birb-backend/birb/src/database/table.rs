use std::{error::Error, fmt::Display};

use async_trait::async_trait;
use sqlx::{FromRow, Postgres, pool::PoolConnection, postgres::PgRow};

// only basic required functionality, custom or additional methods should be added per table or in other traits.
//
// SqlTable is also used as a handle orchestrated and setup by a schema orchestrator, hence the signature (might look weird at first glance but fits for this use case)
#[async_trait]
pub trait SqlTable: Sized {
    type Element: for<'r> FromRow<'r, PgRow> + Send + Sync;
    type InsertedElement: Send + Sync;
    type Error: Error + Display;
    type Identifier: Sync + Send;

    async fn new(conn: PoolConnection<Postgres>) -> Result<Self, Self::Error>;
    async fn insert(
        &mut self,
        element: Self::InsertedElement,
    ) -> Result<Self::Identifier, Self::Error>;
    async fn delete(&mut self, id: Self::Identifier) -> Result<(), Self::Error>;

    async fn get(&mut self, id: Self::Identifier) -> Result<Self::Element, Self::Error>;
}

// used for reuseable table identifier const declarations
#[macro_export]
macro_rules! table_op {
    ($operation:literal, $table:literal, $body:literal) => {
        concat!($operation, " ", $table, " ", $body)
    };

    ($operation:literal, $table:expr, $body:literal) => {
        format!("{} {} {}", $operation, $table, $body)
    };
}
