use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost/postgraphql")
            .await?;

        Ok(AppState { db_pool })
    }
}
