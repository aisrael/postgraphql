use crate::bookstore::Author;
use crate::Error;

#[derive(Debug, Clone)]
pub struct BookStore {
    db_pool: sqlx::pool::Pool<sqlx::Postgres>,
}

impl BookStore {
    pub fn new(db_pool: sqlx::pool::Pool<sqlx::Postgres>) -> BookStore {
        BookStore { db_pool }
    }

    pub async fn list_authors(&self) -> Result<Vec<Author>, Error> {
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

    #[tokio::test]
    async fn test_list_authors() -> Result<(), anyhow::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost/postgraphql")
            .await?;

        let authors = sqlx::query_as::<_, Author>("SELECT id, name FROM authors")
            .fetch_all(&pool)
            .await?;

        let json = json!(authors);
        println!("{}", serde_json::to_string_pretty(&json).unwrap());

        Ok(())
    }
}
