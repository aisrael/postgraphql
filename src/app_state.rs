use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use crate::bookstore::BookStore;

#[derive(Debug, Clone)]
pub struct AppState {
    pub book_store: BookStore
}

impl AppState {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost/postgraphql")
            .await?;

        let book_store = BookStore::new(db_pool);

        Ok(AppState { book_store })
    }
}
