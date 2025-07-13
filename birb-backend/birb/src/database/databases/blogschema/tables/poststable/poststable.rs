use async_trait::async_trait;
use sqlx::{Postgres, pool::PoolConnection, query, query_as, query_scalar};

use crate::{
    database::{databases::error::SchemaError, table::SqlTable},
    server::service::services::blogservice::data::{BlogId, BlogPost, NewBlogPost},
    table_op,
};

pub struct PostsTable {
    conn: PoolConnection<Postgres>,
}

const TABLE: &str = "blog.posts";

#[async_trait]
impl SqlTable for PostsTable {
    type Element = BlogPost;
    type InsertedElement = NewBlogPost;
    type Error = SchemaError;
    type Identifier = BlogId;

    fn new(conn: PoolConnection<Postgres>) -> Self {
        Self { conn }
    }
    async fn insert(
        &mut self,
        element: Self::InsertedElement,
    ) -> Result<Self::Identifier, Self::Error> {
        let id = query_scalar::<_, Self::Identifier>(&table_op!(
            "INSERT INTO",
            TABLE,
            "(title, author, content) VALUES ($1, $2, $3) RETURNING id;"
        ))
        .bind(element.title)
        .bind(element.author)
        .bind(element.content)
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

    async fn get(&mut self, id: Self::Identifier) -> Result<Self::Element, Self::Error> {
        let result = query_as::<_, BlogPost>("SELECT * FROM posts WHERE id = $1;")
            .bind(id)
            .fetch_one(&mut *self.conn)
            .await?;

        Ok(result)
    }
}

impl PostsTable {
    pub async fn get_all(&mut self) -> Result<Vec<BlogPost>, SchemaError> {
        let posts = query_as::<_, BlogPost>("SELECT * FROM posts;")
            .fetch_all(&mut *self.conn)
            .await?;

        Ok(posts)
    }
}
