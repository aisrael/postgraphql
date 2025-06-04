use std::fmt::Debug;
use std::pin::Pin;
use std::sync::LazyLock;
use crate::bookstore::Author;
use crate::Error;
use async_trait::async_trait;
use sqlx::MySqlPool;
use sqlx::postgres::PgPoolOptions;
use crate::app_state::AppState;

pub struct BookStore {
    book_store_impl: Box<dyn BookStoreImpl + Send + Sync>,
}

impl Debug for BookStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BookStore")
    }
}

impl BookStore {

    /// Create and return a BookStore with its BookStoreImpl determined dynamically at run-time
    /// based on the DATABASE_URL
    pub async fn new(database_url: &str) -> Result<BookStore, anyhow::Error> {
        if database_url.starts_with("postgres://") {
            let p = PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await?;
            let book_store_impl = PostgresBookStore::new(p);
            Ok(BookStore {
                book_store_impl: Box::new(book_store_impl)
            })
        } else if database_url.starts_with("mysql://") {
            let m = MySqlPool::connect(database_url).await?;
            let book_store_impl = MySqlBookStore::new(m);
            Ok(BookStore {
                book_store_impl: Box::new(book_store_impl)
            })
        } else {
            Err(Error::DatabaseUrlError(database_url.to_string()).into())
        }
    }

    pub async fn list_authors(&self) -> Result<Vec<Author>, Error> {
        self.book_store_impl.list_authors().await
    }
}

#[async_trait]
pub trait BookStoreImpl {
    async fn list_authors(&self) -> Result<Vec<Author>, Error>;
}

#[derive(Debug, Clone)]
pub struct PostgresBookStore {
    db_pool: sqlx::pool::Pool<sqlx::Postgres>,
}

impl PostgresBookStore {
    pub fn new(db_pool: sqlx::pool::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BookStoreImpl for PostgresBookStore {
    async fn list_authors(&self) -> Result<Vec<Author>, Error> {
        Ok(sqlx::query_as::<_, Author>("SELECT id, name FROM authors")
            .fetch_all(&self.db_pool)
            .await?)
    }
}

#[derive(Debug, Clone)]
pub struct MySqlBookStore {
    db_pool: sqlx::pool::Pool<sqlx::MySql>,
}

impl MySqlBookStore {
    pub fn new(db_pool: sqlx::pool::Pool<sqlx::MySql>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BookStoreImpl for MySqlBookStore {
    async fn list_authors(&self) -> Result<Vec<Author>, Error> {
        Ok(sqlx::query_as::<_, Author>("SELECT id, name FROM authors")
            .fetch_all(&self.db_pool)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::MySqlPool;

    #[tokio::test]
    async fn test_list_authors_postgres() -> Result<(), anyhow::Error> {
        let book_store = BookStore::new("postgres://postgres:postgres@localhost/postgraphql").await?;

        let json = json!(book_store.list_authors().await?);
        println!("{}", serde_json::to_string_pretty(&json).unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_list_authors_mysql() -> Result<(), anyhow::Error> {
        let book_store = BookStore::new("mysql://root:password@localhost/postgraphql").await?;

        let json = json!(book_store.list_authors().await?);
        println!("{}", serde_json::to_string_pretty(&json).unwrap());

        Ok(())
    }
}
