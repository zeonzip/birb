use sqlx::{PgPool, Postgres, pool::PoolConnection, postgres::PgPoolOptions};

const DB_HOST: &'static str = crate::host::HOST;
const MAX_CONNS: u32 = 100; // Server could handle alot more, just for scaling.

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: PgPool,
}

impl DatabaseConnection {
    pub async fn connect() -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(MAX_CONNS)
            .connect(DB_HOST)
            .await?;

        Ok(Self { pool })
    }

    pub async fn acquire_connection(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        Ok(self.pool.acquire().await?)
    }
}
