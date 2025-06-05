use axum::extract::Request;
use axum::middleware::{from_fn, Next};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use tokio_postgres::tls::NoTls;
use tokio_postgres::Client;

mod app_state;
mod auth;
mod bookstore;
mod db_utils;
mod handlers;
pub mod utils;

use crate::app_state::AppState;
use crate::handlers::{authors, get_author_by_id, healthz, list_books_by_author_id};

pub async fn initialize_app() -> Result<axum::Router, anyhow::Error> {
    let app_state = AppState::new().await?;

    let author_routes = Router::new()
        .route("/", get(get_author_by_id))
        .route("/books", get(list_books_by_author_id));

    let router = Router::new()
        .route("/authors", get(authors))
        .nest("/authors/{author_id}", author_routes)
        .route_layer(from_fn(auth))
        .route("/healthz", get(healthz))
        .with_state(app_state);
    Ok(router)
}

async fn auth(request: Request, next: Next) -> Response {
    next.run(request).await
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Don't know how to connect to DATABASE_URL: {0}")]
    DatabaseUrlError(String),
    #[error("postgres error")]
    PostgresError(#[from] tokio_postgres::Error),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
}

pub async fn connect() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgraphql",
        NoTls,
    )
    .await
    .map_err(Error::PostgresError)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

#[cfg(test)]
mod tests {
    use tokio_postgres::SimpleQueryMessage;
    use tokio_postgres::SimpleQueryMessage::{CommandComplete, Row, RowDescription};

    use super::*;

    fn debug(sqm: SimpleQueryMessage) {
        match sqm {
            Row(row) => {
                println!("columns: {:?}", row.columns());
                println!("len: {:?}", row.columns());
                for i in 0..row.columns().len() {
                    println!("column {}: {:?}", i, row.columns()[i]);
                    println!("{}", row.get(i).unwrap());
                }
            }
            CommandComplete(count) => {
                println!("CommandComplete: {:?}", count);
            }
            RowDescription(arc) => {
                println!("RowDescription: {:?}", arc);
                let columns = arc.as_ref();
                for column in columns {
                    println!("Column: {:?}", column);
                }
            }
            _ => panic!("unexpected row kind"),
        }
    }

    #[tokio::test]
    async fn test_pg() -> Result<(), Error> {
        let client = connect().await?;
        let rows = client.simple_query("SELECT 1").await?;
        for row in rows {
            debug(row)
        }
        Ok(())
    }
}
