use async_trait::async_trait;
use sqlx::{Postgres, pool::PoolConnection, query, query_as, query_scalar};

use crate::{
    database::{
        SerialId,
        databases::{
            authschema::tables::admins::data::{Admin, InsertAdmin},
            error::SchemaError,
        },
        table::SqlTable,
    },
    table_op,
};

const TABLE: &str = "auth.admins";

pub struct AdminsTable {
    conn: PoolConnection<Postgres>,
}

#[async_trait]
impl SqlTable for AdminsTable {
    type Element = Admin;
    type InsertedElement = InsertAdmin;
    type Error = SchemaError;
    type Identifier = SerialId;

    fn new(conn: PoolConnection<Postgres>) -> Self {
        Self { conn }
    }

    async fn get(&mut self, id: Self::Identifier) -> Result<Self::Element, Self::Error> {
        let element = query_as::<_, Admin>(&table_op!("SELECT * FROM", TABLE, "WHERE id = $1;"))
            .bind(id)
            .fetch_one(&mut *self.conn)
            .await?;

        Ok(element)
    }

    async fn insert(
        &mut self,
        element: Self::InsertedElement,
    ) -> Result<Self::Identifier, Self::Error> {
        let id = query_scalar::<_, Self::Identifier>(&table_op!(
            "INSERT INTO",
            TABLE,
            "(username, hashed_pass, salt, issued_at) VALUES ($1, $2, $3) RETURNING id;"
        ))
        .bind(element.username)
        .bind(element.hashed_pass)
        .bind(element.salt)
        .bind(element.issued_by)
        .fetch_one(&mut *self.conn)
        .await?;

        Ok(id)
    }

    async fn delete(&mut self, id: Self::Identifier) -> Result<(), Self::Error> {
        query("DELETE FROM posts WHERE id = $1;")
            .bind(id)
            .execute(&mut *self.conn)
            .await?;

        Ok(())
    }
}
