use std::fmt::Debug;
use crate::bookstore::bookstore::{MySqlBookStore, PostgresBookStore};
use sqlx::postgres::PgPoolOptions;
use sqlx::MySqlPool;
use std::sync::{Arc, LazyLock};
use crate::bookstore::BookStore;

pub static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("DATABASE_URL").expect("DATABASE_URL not set!"));

#[derive(Clone)]
pub struct AppState {
    pub book_store: Arc<BookStore>,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

impl AppState {
    pub async fn new() -> Result<AppState, anyhow::Error> {
        let book_store = Arc::new(BookStore::new(&DATABASE_URL).await?);
        Ok(AppState { book_store })
    }
}
