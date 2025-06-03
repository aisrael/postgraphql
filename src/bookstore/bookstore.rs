#[derive(Debug)]
pub struct BookStore {}

use crate::bookstore::Author;
use crate::Error;

impl BookStore {
    pub async fn list_authors() -> Result<Vec<Author>, Error> {
        Ok(vec![])
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
